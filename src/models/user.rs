use serde::{Deserialize, Serialize};

/// Database models for a user
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

