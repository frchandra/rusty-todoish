use dotenvy::dotenv;
use rusty_todoish::app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    app::app::run().await;
}
