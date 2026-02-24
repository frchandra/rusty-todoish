use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::app::errors::AppErrorCode;
use crate::app::services::notes_services;
use crate::{
    app::state::AppState,
    models::note::NoteModel,
    rest::schemas::note_schemas::{
        CreateNoteSchema, FilterOptions, NoteModelResponse, UpdateNoteSchema,
    },
};

pub async fn note_list_handler(
    Query(opts): Query<FilterOptions>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Param
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let notes = notes_services::list_notes(&app_state, limit as i64, offset as i64)
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

    let json_response = serde_json::json!(note_responses);

    Ok(Json(json_response))
}

// implement get_note_handler
pub async fn get_note_handler(
    Path(note_id): Path<uuid::Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note = notes_services::get_note_by_id(&app_state, note_id)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "message": format!("{}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let note_response = to_note_response(&note);

    Ok(Json(serde_json::json!(note_response)))
}

pub async fn create_note_handler(
    State(app_state): State<AppState>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let note = notes_services::create_note(&app_state, &body.title, &body.content, body.is_published)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "message": format!("{}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let note_response = to_note_response(&note);

    Ok((StatusCode::OK, Json(serde_json::json!(note_response))))
}

pub async fn update_note_handler(
    Path(note_id): Path<uuid::Uuid>,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    notes_services::update_note_by_id(
        &app_state,
        note_id,
        body.title,
        body.content,
        body.is_published,
    )
    .await
    .map_err(|e| {
        let status_code = match e.error_code {
            AppErrorCode::ResourceNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_response = serde_json::json!({
            "message": format!("{}", e),
        });
        (status_code, Json(error_response))
    })?;

    Ok(StatusCode::NO_CONTENT)
}

fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
    }
}
