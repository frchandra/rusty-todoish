use crate::app::errors::{AppError, AppErrorCode, ErrorEntry};
use crate::app::repositories::notes_repositories;
use crate::app::state::AppState;
use crate::models::note::NoteModel;


pub async fn list_notes(
    app_state: &AppState,
    limit: i64,
    offset: i64,
) -> Result<Vec<NoteModel>, AppError> {
    let notes = notes_repositories::list_notes(&app_state, limit as i64, offset as i64).await.map_err(AppError::from)?;
    Ok(notes)
}

pub async fn get_note_by_id(
    app_state: &AppState,
    note_id: uuid::Uuid,
) -> Result<NoteModel, AppError> {
    let note = notes_repositories::get_note_by_id(&app_state, note_id)
        .await
        //if there is any error, pass it to AppError::from(e)
        .map_err(AppError::from)?;
    Ok(note)
}

pub async fn create_note(
    app_state: &AppState,
    title: &str,
    content: &str,
    is_published: Option<bool>,
) -> Result<NoteModel, AppError> {
    let note = notes_repositories::create_note(
        &app_state,
        title,
        content,
        is_published.unwrap_or(false),
    )
    .await
    .map_err(AppError::from)?;

    Ok(note)
}

pub async fn update_note_by_id(
    app_state: &AppState,
    note_id: uuid::Uuid,
    title: Option<String>,
    content: Option<String>,
    is_published: Option<bool>,
) -> Result<NoteModel, AppError> {
    let note = notes_repositories::update_note_by_id(
        &app_state,
        note_id,
        title,
        content,
        is_published,
    )
    .await
    .map_err(AppError::from)?;

    Ok(note)
}

pub async fn delete_note_by_id(
    app_state: &AppState,
    note_id: uuid::Uuid,
) -> Result<(), AppError> {
    let rows_affected = notes_repositories::delete_note_by_id(&app_state, note_id)
        .await
        .map_err(AppError::from)?;

    if rows_affected == 0 {
        return Err(AppError::new(
            AppErrorCode::ResourceNotFound,
            ErrorEntry::new("note not found"),
        ));
    }

    Ok(())
}
