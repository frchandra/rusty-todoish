use crate::models::note::NoteModel;
use sqlx::PgPool;

pub async fn list_notes(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<NoteModel>, sqlx::Error> {
    sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes ORDER BY id LIMIT $1 OFFSET $2"#,
        limit,
        offset
    )
        .fetch_all(pool)
        .await
}