use axum::{
	extract::State,
	http::StatusCode,
	response::IntoResponse,
	Json,
};

use crate::app::errors::AppErrorCode;
use crate::app::services::users_services;
use crate::app::state::AppState;
use crate::models::user::UserModel;
use crate::rest::schemas::user_schemas::{LoginUserSchema, UserModelResponse};

pub async fn login_handler(
	State(app_state): State<AppState>,
	Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
	let user = users_services::login(&app_state, &body.name, &body.password)
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

	let user_response = to_user_response(&user);

	Ok((StatusCode::OK, Json(serde_json::json!(user_response))))
}

fn to_user_response(user: &UserModel) -> UserModelResponse {
	UserModelResponse {
		id: user.id,
		name: user.name.to_owned(),
		email: user.email.to_owned(),
		is_admin: user.is_admin,
	}
}
