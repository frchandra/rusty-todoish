use dotenvy::dotenv;
use reqwest::StatusCode;
use std::env;
use std::time::{Duration, Instant};

use rusty_todoish::app;

pub async fn start_server() {
    dotenv().ok();
    println!("Starting server...");
    let (server, listener) = app::server::build_server_and_listener()
        .await
        .expect("Failed to build app and listener");
    //run the server
    tokio::spawn(async move {
        axum::serve(listener, server.into_make_service())
            .await
            .expect("Failed to run the server");
    });
    println!("{:?}", Instant::now());
    wait_for_service(Duration::from_secs(30)).await;
}

async fn wait_for_service(duration: Duration) {
    let timeout = Instant::now() + duration;
    let service_host = env::var("SERVICE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let service_port = env::var("SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
    let url = format!("http://{}:{}/", service_host, service_port);
    loop {
        if let Ok(response) = reqwest::get(&url).await
            && response.status() == StatusCode::OK
        {
            break;
        }
        if Instant::now() > timeout {
            panic!("Could not start API Server in: {:?}", duration);
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}
