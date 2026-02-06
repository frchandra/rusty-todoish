use crate::app::config::AppConfig;
use crate::infra::postgres::instance::create_instance;
use crate::rest::routes::health_check_routes;
use axum::Router;
use tokio::net::TcpListener;
use crate::app::state::AppState;

pub async fn run() {
    let app_config = AppConfig::from_env();
    let db_pool = create_instance(app_config.database_url.as_str()).await;

    let bind_addr = app_config.bind_addr.clone();

    // let app_state = AppState { app_config, db_pool };

    let app = Router::new()
        .merge(health_check_routes::routes());


    let listener = TcpListener::bind(bind_addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
