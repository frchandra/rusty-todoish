use axum::ServiceExt;
use dotenvy::dotenv;
use tokio::net::TcpListener;

use rusty_todoish::app;


#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    // let config = AppConfig::from_env();
    // let pool = create_instance(&config.database_url).await;

    app::app::run().await;

    // println!("Server started at {}", config.bind_addr);


}
