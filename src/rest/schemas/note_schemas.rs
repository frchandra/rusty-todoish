use serde::{Deserialize, Serialize};

/// Query parameters for listing notes with pagination
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

/// JSON response models for a note
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// /// Schema for creating a new note
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreateNoteSchema {
//     pub title: String,
//     pub content: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub is_published: Option<bool>,
// }
//
// /// Schema for updating an existing note
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateNoteSchema {
//     pub title: Option<String>,
//     pub content: Option<String>,
//     pub is_published: Option<bool>,
// }
