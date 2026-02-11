use crate::app::state::AppState;
use crate::models::note::NoteModel;

pub async fn list_notes(
    app_state: &AppState,
    limit: i64,
    offset: i64,
) -> Result<Vec<NoteModel>, sqlx::Error> {
    let notes = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes ORDER BY id LIMIT $1 OFFSET $2"#,
        limit,
        offset
    )
    .fetch_all(&app_state.db_pool)
    .await?;

    Ok(notes)
}
