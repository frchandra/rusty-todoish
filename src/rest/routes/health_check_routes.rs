use axum::{
    Router,
    routing::get
};

use crate::{
    rest::handlers::health_check_handlers::{
        health_check_handler
    }
};

pub fn routes() -> Router<()> {
    Router::new()
        .route("/rest/v1/health_check", get(health_check_handler))
}
