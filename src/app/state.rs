// use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::app::config::AppConfig;

pub struct AppState {
    pub db_pool: PgPool,
    pub app_config: AppConfig,
}

