use crate::app::config::AppConfig;
use redis::aio::MultiplexedConnection;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub app_config: AppConfig,
    pub redis_connection: MultiplexedConnection,
}
