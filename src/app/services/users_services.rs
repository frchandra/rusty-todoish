use crate::app::errors::{AppError, AppErrorCode};
use crate::app::repositories::users_repositories;
use crate::app::state::AppState;
use crate::models::user::UserModel;

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
