use std::error::Error;
use crate::app::errors;
use crate::app::repositories::notes_repositories;
use crate::app::state::AppState;
use crate::models::note::NoteModel;

pub async fn list_notes(
    app_state: &AppState,
    limit: i64,
    offset: i64,
) -> Result<Vec<NoteModel>, errors::AppError> {
    let notes = notes_repositories::list_notes(&app_state, limit as i64, offset as i64)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                let error_message = format!("No notes found for limit {} and offset {}", limit, offset);
                ()
            }
            _ => e,
        })?;

    // .await
    //     .map_err(|e| match e {
    //         sqlx::Error::RowNotFound => {
    //             let user_error = UserError::UserNotFound(id);
    //             (user_error.status_code(), APIErrorEntry::from(user_error)).into()
    //         }
    //         _ => APIError::from(e),
    //     })?;

    Ok(notes)
}
