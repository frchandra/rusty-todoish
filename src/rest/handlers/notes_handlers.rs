use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::app::services::notes_services;
use crate::{
    app::errors::AppError,
    rest::sessions::token::{AccessToken, Claimable},
};
use crate::{
    app::state::AppState,
    models::note::NoteModel,
    rest::schemas::note_schemas::{
        CreateNoteSchema, FilterOptions, NoteModelResponse, UpdateNoteSchema,
    },
};

pub async fn note_list_handler(
    access_claims: AccessToken,
    Query(opts): Query<FilterOptions>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    access_claims.validate_role_admin_or_user()?;
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let notes = notes_services::list_notes(&app_state, limit as i64, offset as i64).await?;
    let note_responses = notes.iter().map(to_note_response).collect::<Vec<_>>();
    Ok(Json(serde_json::json!(note_responses)))
}

// implement get_note_handler
pub async fn get_note_handler(
    access_claims: AccessToken,
    Path(note_id): Path<uuid::Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    access_claims.validate_role_admin_or_user()?;
    let note = notes_services::get_note_by_id(&app_state, note_id).await?;
    let note_response = to_note_response(&note);
    Ok(Json(serde_json::json!(note_response)))
}

pub async fn create_note_handler(
    access_claims: AccessToken,
    State(app_state): State<AppState>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, AppError> {
    access_claims.validate_role_admin()?;
    let note =
        notes_services::create_note(&app_state, &body.title, &body.content, body.is_published)
            .await?;
    let note_response = to_note_response(&note);
    Ok((StatusCode::OK, Json(serde_json::json!(note_response))))
}

pub async fn update_note_handler(
    access_claims: AccessToken,
    Path(note_id): Path<uuid::Uuid>,
    State(app_state): State<AppState>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, AppError> {
    access_claims.validate_role_admin()?;
    notes_services::update_note_by_id(
        &app_state,
        note_id,
        body.title,
        body.content,
        body.is_published,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_note_handler(
    Path(note_id): Path<uuid::Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    notes_services::delete_note_by_id(&app_state, note_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_then_update_note_handler(
    State(app_state): State<AppState>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, AppError> {
     let note =
        notes_services::add_then_update_note(&app_state, &body.title, &body.content, body.is_published)
            .await?;
    let note_response = to_note_response(&note);
    Ok((StatusCode::OK, Json(serde_json::json!(note_response))))
}


fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id,
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
    }
}
