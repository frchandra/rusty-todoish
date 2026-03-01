use crate::app::state::AppState;
use crate::models::user::UserModel;

// implementation of get user by name and password.
pub async fn get_user_by_name_and_password(
    app_state: &AppState,
    name: &str,
    password: &str,
) -> Result<UserModel, sqlx::Error> {
    let user = sqlx::query_as!(
        UserModel,
        r#"SELECT * FROM users WHERE name = $1 AND password = $2"#,
        name,
        password
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(user)
}


