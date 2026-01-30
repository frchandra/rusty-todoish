use dotenvy::dotenv;
use tokio::net::TcpListener;

use rusty_todoish::app::config::AppConfig;
use rusty_todoish::app::rest_app::create_rest_app;
use rusty_todoish::infra::postgres::instance::create_instance;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    let config = AppConfig::from_env();
    let pool = create_instance(&config.database_url).await;

    let app = create_rest_app(pool);

    println!("Server started at {}", config.bind_addr);

    let listener = TcpListener::bind(&config.bind_addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
