use axum::Router;
use sqlx::PgPool;
use crate::rest::routes::health_check_routes;

pub fn create_rest_app(pg_pool: PgPool) -> Router {
    // pool can be added to state later
    health_check_routes::routes()
}