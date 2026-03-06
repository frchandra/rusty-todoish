use crate::app::state::AppState;
use crate::models::user::UserModel;

// implementation of get user by email and password.
pub async fn get_user_by_email_and_password(
    app_state: &AppState,
    email: &str,
    password: &str,
) -> Result<UserModel, sqlx::Error> {
    let user = sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM users WHERE email = $1 AND password = $2"#,
        email,
        password
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(user)
}


