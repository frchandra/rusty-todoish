use serde::{Deserialize, Serialize};

// JSON response models for a user
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModelResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserSchema {
    pub name: String,
    pub password: String,
}
