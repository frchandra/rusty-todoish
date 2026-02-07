use crate::app::config::AppConfig;
use crate::app::state::AppState;
use crate::infra::postgres::instance::create_instance;
use crate::rest::routes::{health_check_routes, notes_routes};
use axum::Router;
use tokio::net::TcpListener;

pub async fn build_server_and_listener() -> Result<(Router, TcpListener), std::io::Error> {
    let app_config = AppConfig::from_env();
    let db_pool = create_instance(app_config.database_url.as_str()).await;

    let bind_addr = app_config.bind_addr.clone();

    let app_state = AppState { app_config, db_pool };

    let app = Router::new()
        .merge(health_check_routes::routes())
        .merge(notes_routes::routes())
        .with_state(app_state);

    let listener = TcpListener::bind(bind_addr).await?;

    Ok((app, listener))
}
