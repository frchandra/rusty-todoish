mod common;
mod notes_test;

use common::test_server;
use reqwest::StatusCode;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn health_check_test() {
    // Start rest server
    test_server::start_server().await;

    let url = "http://127.0.0.1:8080/api/v1/health_check";
    let response = reqwest::get(url).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("Message log : {:#?}", json["message"]);

    assert_eq!(json["message"], "/rest/v1");
}

#[tokio::test]
#[serial]
async fn list_notes_test() {
    // Start rest server
    test_server::start_server().await;

    let url = "http://127.0.0.1:8080/api/v1/notes";
    let response = reqwest::get(url).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
