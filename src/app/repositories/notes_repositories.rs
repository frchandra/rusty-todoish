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

pub async fn get_note_by_id(
    app_state: &AppState,
    note_id: uuid::Uuid,
) -> Result<NoteModel, sqlx::Error> {
    let note = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes WHERE id = $1::uuid"#,
        note_id
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(note)
}
