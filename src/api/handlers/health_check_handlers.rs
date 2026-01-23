use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "/api/v1"; //TODO: dont put literals in the source code

    let json_response = json!({
        "message": MESSAGE,
    });

    (StatusCode::OK, Json(json_response))
}
