use axum::{
    Router,
    routing::get
};

use crate::{
    rest::handlers::health_check_handlers::{
        health_check_handler
    }
};

use crate::app::state::AppState;

pub fn routes() -> Router<(AppState)> {
    Router::new()
        .route("/api/v1/health_check", get(health_check_handler))
}
