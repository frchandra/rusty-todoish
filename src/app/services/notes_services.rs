use crate::app::errors::AppError;
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
