use axum::{
	extract::State,
	http::StatusCode,
	response::IntoResponse,
	Json,
};
use serde_json::json;
use crate::app::errors::AppErrorCode;
use crate::app::services::users_services;
use crate::app::state::AppState;
use crate::models::user::UserModel;
use crate::rest::schemas::user_schemas::{LoginUserSchema, UserModelResponse};
use crate::rest::sessions::auth_utils;
use crate::rest::sessions::auth_utils::AuthTokens;

pub async fn login_handler(
	State(app_state): State<AppState>,
	Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let user = users_services::login(&app_state, &body.email, &body.password)
		.await
		.map_err(|e| {
			let status_code = match e.error_code {
				AppErrorCode::AuthenticationWrongCredentials => StatusCode::UNAUTHORIZED,
				_ => StatusCode::INTERNAL_SERVER_ERROR,
			};
			let error_response = serde_json::json!({
				"message": format!("{}", e),
			});
			(status_code, Json(error_response))
		})?;

    let tokens =  auth_utils::generate_tokens(user, &app_state.app_config);

	Ok((StatusCode::OK, tokens_to_response(tokens)))
}

fn tokens_to_response(jwt_tokens: AuthTokens) -> impl IntoResponse {
    let json = json!({
        "access_token": jwt_tokens.access_token,
        "refresh_token": jwt_tokens.refresh_token,
        "token_type": "Bearer"
    });

    Json(json)
}
