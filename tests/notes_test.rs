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

//create a test function for populating notes table using dummy_factory
#[tokio::test]
async fn populate_notes_table_test() {
    // Populate notes table
    dummy_factory::populate_notes().await.unwrap();
}

//create a test function for clearing notes table using dummy_factory
#[tokio::test]
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


// Create a test unit for testing get one note by id endpoint /notes/{id}
// here is the flow
// get all notes and get the id (uuid) of the first note
// then call the endpoint /notes/{id} with the id of the first note
// then assert that the response status is 200 OK
// then just print the response body in json format in terminal
#[tokio::test]
#[serial]
async fn get_note_by_id_test() {
    test_server::start_server().await;

    let url = format!("{}/notes", &*ROOT_URL);

    let response = reqwest::get(&url).await.expect("request failed");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body = response.text().await.expect("failed to read response body");

    // Parse JSON
    let json: Value = serde_json::from_str(&body).expect("response is not valid JSON");

    // Get the first note's id
    let first_note_id = json[0]["id"].as_str().unwrap();

    print!("First note id: {first_note_id}\n");

    // Call endpoint /notes/{id} with the id of the first note
    let url = format!("{}/notes/{}", &*ROOT_URL, first_note_id);
    let response = reqwest::get(&url).await.expect("request failed");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body = response.text().await.expect("failed to read response body");

    // Parse JSON
    let json: Value = serde_json::from_str(&body).expect("response is not valid JSON");

    // Pretty print
    let pretty = serde_json::to_string_pretty(&json).unwrap();

    println!("{}", pretty);
}

#[tokio::test]
#[serial]
async fn create_note_test() {
    test_server::start_server().await;

    let url = format!("{}/notes", &*ROOT_URL);

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let randomized_title = format!("My new note {}", nonce);
    let randomized_content = format!("Hello from integration test {}", nonce);
    let randomized_is_published = nonce % 2 == 0;

    let payload = serde_json::json!({
        "title": randomized_title,
        "content": randomized_content,
        "is_published": randomized_is_published
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

    assert_eq!(json["title"].as_str().unwrap(), randomized_title);
    assert_eq!(json["content"].as_str().unwrap(), randomized_content);
    assert_eq!(json["is_published"].as_bool().unwrap(), randomized_is_published);
    assert!(json["id"].as_str().is_some());
}