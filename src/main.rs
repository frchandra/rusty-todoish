mod api;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = api::routes::health_check_routes::routes();
    //create router from the router module

    println!("Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}