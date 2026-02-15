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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check_handler))
}
