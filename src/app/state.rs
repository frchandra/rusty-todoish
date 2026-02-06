use sqlx::postgres::{PgPool, /*PgPoolOptions*/};
use crate::app::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub app_config: AppConfig,
}

