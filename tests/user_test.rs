pub mod common;

use common::dummy_factory;
use crate::common::test_server;
use serial_test::serial;
use serde_json::Value;
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
        "name": "admin_user",
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

