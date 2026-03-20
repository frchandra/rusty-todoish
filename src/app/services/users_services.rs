use crate::app::errors::{AppError, AppErrorCode};
use crate::app::repositories::users_repositories;
use crate::app::state::AppState;
use crate::models::user::UserModel;
use crate::rest::sessions::token::{RefreshToken, TokenType, revoke_refresh_token, validate_token_type};

pub async fn login(
	app_state: &AppState,
	email: &str,
	password: &str,
) -> Result<UserModel, AppError> {
	let user = users_repositories::get_user_by_email_and_password(app_state, email, password)
		.await
		.map_err(|err| match err {
			sqlx::Error::RowNotFound => AppError::new(
				AppErrorCode::AuthenticationWrongCredentials,
				"wrong credentials",
			),
			_ => AppError::from(err),
		})?;

	Ok(user)
}

pub async fn logout(refresh_claims: RefreshToken, state: AppState) -> Result<(), AppError> {
    // Check if revoked tokens are enabled.
    if state.app_config.jwt_enable_revoked_tokens {
        Err(AppError::new(AppErrorCode::AuthenticationRevokedTokensInactive, "Revoked tokens are not enabled"))?
    }

    // Decode and validate the refresh token.
    if !validate_token_type(&refresh_claims, TokenType::RefreshToken) {
        return Err(AppError::new(AppErrorCode::AuthenticationInvalidToken, "Invalid token"));
    }
    revoke_refresh_token(&refresh_claims, &state).await?;
    Ok(())
}
