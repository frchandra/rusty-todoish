use dotenvy::dotenv;
use rusty_todoish::app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    let (server, listener) = app::server::build_server_and_listener()
        .await
        .expect("Failed to build app and listener");
    //run the server
    axum::serve(listener, server.into_make_service()).await.expect("Failed to run the server");
}
