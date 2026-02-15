use crate::app::state::AppState;
use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn health_check_handler(State(app_state): State<AppState>) -> impl IntoResponse {
    let json_response = json!({
        "service_name": app_state.app_config.service_name,
        "service_version": app_state.app_config.service_version,
    });
    (StatusCode::OK, Json(json_response))
}
