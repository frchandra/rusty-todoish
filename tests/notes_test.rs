pub mod common;

use crate::common::test_server;
use common::dummy_factory;
use serial_test::serial;
use std::{env, sync::LazyLock};

static ROOT_URL: LazyLock<String> = LazyLock::new(|| {
    let service_host = env::var("SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("http://{}:{}", service_host, service_port)
});

#[tokio::test]
//create a test function for populating notes table using dummy_factory
async fn populate_notes_table_test() {
    // Populate notes table
    dummy_factory::populate_notes().await.unwrap();
}

#[tokio::test]
//create a test function for clearing notes table using dummy_factory
async fn clear_notes_table_test() {
    // Clear notes table
    dummy_factory::clear_notes().await.unwrap();
}

use serde_json::Value;

#[tokio::test]
#[serial]
async fn list_notes_test() {
    test_server::start_server().await;

    let url = format!("{}/notes", &*ROOT_URL);

    let response = reqwest::get(&url).await.expect("request failed");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body = response.text().await.expect("failed to read response body");

    // Parse JSON
    let json: Value = serde_json::from_str(&body).expect("response is not valid JSON");

    // Pretty print
    let pretty = serde_json::to_string_pretty(&json).unwrap();

    // Optional: trim to avoid clutter
    let preview_len = 1000;
    let preview = pretty.chars().take(preview_len).collect::<String>();

    println!("{preview}");

    if pretty.len() > preview_len {
        println!("... (trimmed)");
    }
}
