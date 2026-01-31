use std::time::{Duration, Instant};
use dotenvy::dotenv;
use reqwest::StatusCode;
use tokio::net::TcpListener;
use rusty_todoish::app::config::AppConfig;
use rusty_todoish::app::rest_app::create_rest_app;
use rusty_todoish::infra::postgres::instance::create_instance;

pub async fn start_server() {
    dotenv().ok();
    println!("Starting nserver...");

    let config = AppConfig::from_env();
    let pool = create_instance(&config.database_url).await;

    let app = create_rest_app(pool);
    println!("{:?}", Instant::now());

    // Run the rest server
    let listener = TcpListener::bind(&config.bind_addr).await.unwrap();



    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service()).await;
    });
    println!("{:?}", Instant::now());
    wait_for_service(Duration::from_secs(30)).await;

    println!("Server started at {}", config.bind_addr);
}

async fn wait_for_service(duration: Duration) {
    let timeout = Instant::now() + duration;
    loop {
        let url = "http://127.0.0.1:8080/api/v1/health_check";
        if let Ok(response) = reqwest::get(url).await
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