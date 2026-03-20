pub mod common;

use crate::common::test_server;
use common::dummy_factory;
use reqwest::header;
use serde_json::Value;
use serial_test::serial;
use std::{env, sync::LazyLock};

static ROOT_URL: LazyLock<String> = LazyLock::new(|| {
    let service_host = env::var("SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("http://{}:{}", service_host, service_port)
});

//create a test function for populating users table using dummy_factory
#[tokio::test]
async fn populate_users_table_test() {
    // Populate users table
    dummy_factory::populate_users().await.unwrap();
}

//create a test function for clearing users table using dummy_factory
#[tokio::test]
async fn clear_users_table_test() {
    // Clear users table
    dummy_factory::clear_users().await.unwrap();
}

#[tokio::test]
#[serial]
async fn login_user_test() {
    dummy_factory::clear_users().await.unwrap();
    dummy_factory::populate_users().await.unwrap();
    test_server::start_server().await;

    let url = format!("{}/user/login", &*ROOT_URL);

    let payload = serde_json::json!({
        "email": "admin@example.com",
        "password": "admin_password"
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .expect("request failed");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body = response.text().await.expect("failed to read response body");
    let json: Value = serde_json::from_str(&body).expect("response is not valid JSON");

    let pretty = serde_json::to_string_pretty(&json).unwrap();
    println!("{}", pretty);
}

#[tokio::test]
#[serial]
async fn logout_user_test() {
    dummy_factory::clear_users().await.unwrap();
    dummy_factory::populate_users().await.unwrap();
    test_server::start_server().await;

    let client = reqwest::Client::new();
    let notes_url = format!("{}/notes", &*ROOT_URL);

    // 1) Access /notes without token -> should be rejected.
    let before_login_response = client
        .get(&notes_url)
        .send()
        .await
        .expect("request failed");
    assert_eq!(before_login_response.status(), reqwest::StatusCode::UNAUTHORIZED);

    // 2) Login with admin credentials.
    let login_url = format!("{}/user/login", &*ROOT_URL);
    let payload = serde_json::json!({
        "email": "admin@example.com",
        "password": "admin_password"
    });

    let login_response = client
        .post(&login_url)
        .json(&payload)
        .send()
        .await
        .expect("login request failed");
    assert_eq!(login_response.status(), reqwest::StatusCode::OK);

    let login_body = login_response
        .text()
        .await
        .expect("failed to read login response body");
    let login_json: Value = serde_json::from_str(&login_body).expect("response is not valid JSON");

    let access_token = login_json["access_token"]
        .as_str()
        .expect("missing access_token in login response");
    let refresh_token = login_json["refresh_token"]
        .as_str()
        .expect("missing refresh_token in login response");

    // 3) Access /notes with access token -> should be allowed.
    let after_login_response = client
        .get(&notes_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await
        .expect("request failed");
    assert_eq!(after_login_response.status(), reqwest::StatusCode::OK);

    // 4) Logout with refresh token.
    let logout_url = format!("{}/user/logout", &*ROOT_URL);
    let logout_response = client
        .post(&logout_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", refresh_token))
        .send()
        .await
        .expect("logout request failed");

    let logout_status = logout_response.status();
    let logout_body = logout_response
        .text()
        .await
        .expect("failed to read logout response body");

    println!("logout status: {}", logout_status);
    println!("logout body: {}", logout_body);

    assert_eq!(logout_status, reqwest::StatusCode::OK);

    // 5) Access /notes again using the same access token -> should be rejected.
    let after_logout_response = client
        .get(&notes_url)
        .header(header::AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await
        .expect("request failed");

    let after_logout_status = after_logout_response.status();
    assert!(
        matches!(
            after_logout_status,
            reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::BAD_REQUEST
        ),
        "expected unauthorized or bad request after logout, got {}",
        after_logout_status
    );
}



