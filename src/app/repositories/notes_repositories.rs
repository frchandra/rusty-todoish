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

pub async fn create_note(
    app_state: &AppState,
    title: &str,
    content: &str,
    is_published: bool,
) -> Result<NoteModel, sqlx::Error> {
    let note = sqlx::query_as!(
        NoteModel,
        r#"
        INSERT INTO notes (title, content, is_published)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        title,
        content,
        is_published
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(note)
}

pub async fn update_note_by_id(
    app_state: &AppState,
    note_id: uuid::Uuid,
    title: Option<String>,
    content: Option<String>,
    is_published: Option<bool>,
) -> Result<NoteModel, sqlx::Error> {
    let note = sqlx::query_as!(
        NoteModel,
        r#"
        UPDATE notes
        SET title = COALESCE($2, title),
            content = COALESCE($3, content),
            is_published = COALESCE($4, is_published)
        WHERE id = $1::uuid
        RETURNING *
        "#,
        note_id,
        title,
        content,
        is_published
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(note)
}
