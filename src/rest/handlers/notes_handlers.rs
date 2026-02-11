use axum::{
    Json,
    extract::{/*Path,*/ Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::app::repositories::notes_repositories;
use crate::{
    app::state::AppState,
    models::note::{NoteModel, NoteModelResponse},
    rest::schemas::note_schemas::{/*CreateNoteSchema,*/ FilterOptions, /*UpdateNoteSchema*/},
};

pub async fn note_list_handler(
    Query(opts): Query<FilterOptions>,
    State(data): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let notes = notes_repositories::list_notes(&data, limit as i64, offset as i64)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    // Response
    let note_responses = notes
        .iter()
        .map(to_note_response)
        .collect::<Vec<NoteModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": note_responses.len(),
        "notes": note_responses
    });

    Ok(Json(json_response))
}

// Convert DB Model to Response
fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published != 0,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
    }
}
