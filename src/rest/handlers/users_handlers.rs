use axum::{
	extract::State,
	http::StatusCode,
	response::IntoResponse,
	Json,
};
use serde_json::json;
use crate::app::errors::{AppError};
use crate::app::services::users_services;
use crate::app::state::AppState;
use crate::rest::schemas::user_schemas::{LoginUserSchema};
use crate::rest::sessions::token;
use crate::rest::sessions::token::{AccessToken, Claimable, TokenPair};

pub async fn login_handler(
	State(app_state): State<AppState>,
	Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, AppError> {
	let user = users_services::login(&app_state, &body.email, &body.password).await?;
    let tokens =  token::generate_tokens(user, &app_state.app_config);
	Ok((StatusCode::OK, tokens_to_response(tokens)))
}

pub async fn revoke_all_handler(
    State(state): State<AppState>,
    access_claims: AccessToken,
) -> Result<impl IntoResponse, AppError> {
    access_claims.validate_role_admin()?;
    token::revoke_global(&state).await?;
    Ok(())
}

fn tokens_to_response(jwt_tokens: TokenPair) -> impl IntoResponse {
    let json = json!({
        "access_token": jwt_tokens.access_token,
        "refresh_token": jwt_tokens.refresh_token,
        "token_type": "Bearer"
    });

    Json(json)
}
