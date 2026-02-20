pub mod common;

use common::test_server;
use reqwest::StatusCode;
use serial_test::serial;
use std::{env, sync::LazyLock};

static HEALTH_CHECK_URL: LazyLock<String> = LazyLock::new(|| {
    let service_host = env::var("SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("http://{}:{}", service_host, service_port)
});

#[tokio::test]
#[serial]
async fn health_check_test() {
    // Start rest server
    test_server::start_server().await;

    let response = reqwest::get(&*HEALTH_CHECK_URL).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("Message log : {:#?}", json["message"]);

    assert_eq!(json["service_name"], "rusty-todoish");
}