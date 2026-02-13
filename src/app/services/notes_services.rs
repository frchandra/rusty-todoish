use crate::app::repositories::notes_repositories;
use crate::app::state::AppState;
use crate::models::note::NoteModel;

pub async fn list_notes(
    app_state: &AppState,
    limit: i64,
    offset: i64,
) -> Result<Vec<NoteModel>, sqlx::Error> {
    let notes = notes_repositories::list_notes(&app_state, limit as i64, offset as i64)
        .await
        // .map_err(|e| {
        //     let error_response = serde_json::json!({
        //         "status": "error",
        //         "message": format!("Database error: {}", e),
        //     });
        //     (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        // })
        ?;

    Ok(notes)
}
