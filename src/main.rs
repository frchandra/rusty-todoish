use dotenvy::dotenv;
use tokio::signal;
use tokio::signal::unix;
use tokio::signal::unix::SignalKind;
use rusty_todoish::app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    let (server, listener) = app::server::build_server_and_listener()
        .await
        .expect("Failed to build app and listener");
    //run the server
    axum::serve(listener, server.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to run the server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("received termination signal, shutting down...");
}