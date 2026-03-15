use crate::app::config::AppConfig;
use std::sync::Arc;
use sqlx::postgres::PgPool;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub app_config: AppConfig,
    pub redis_connection: Arc<Mutex<redis::aio::MultiplexedConnection>>,
}
