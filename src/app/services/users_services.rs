use crate::app::errors::{AppError, AppErrorCode, ErrorEntry};
use crate::app::repositories::users_repositories;
use crate::app::state::AppState;
use crate::models::user::UserModel;

pub async fn login(
	app_state: &AppState,
	name: &str,
	password: &str,
) -> Result<UserModel, AppError> {
	let user = users_repositories::get_user_by_name_and_password(app_state, name, password)
		.await
		.map_err(|err| match err {
			sqlx::Error::RowNotFound => AppError::new(
				AppErrorCode::AuthenticationWrongCredentials,
				ErrorEntry::new("wrong credentials"),
			),
			_ => AppError::from(err),
		})?;

	Ok(user)
}
