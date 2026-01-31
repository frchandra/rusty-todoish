mod common;

use common::rest_app_test;
use reqwest::StatusCode;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn health_check_test() {
    // Start rest server
    rest_app_test::start_server().await;

    let url = "http://127.0.0.1:8080/api/v1/health_check";
    let response = reqwest::get(url).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("AAAA {:#?}", json["message"]);

    assert_eq!(json["message"], "/rest/v1");
}
