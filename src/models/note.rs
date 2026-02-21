use serde::{Deserialize, Serialize};

/// Database models for a note
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}