use sqlx::PgPool;
use std::env;
use std::sync::LazyLock;

static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    let _ = dotenvy::dotenv();
    env::var("DATABASE_URL").unwrap_or_else(|_| {
        let db = env::var("POSTGRES_DATABASE_NAME").expect("POSTGRES_DATABASE_NAME missing");
        let user = env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME missing");
        let pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing");
        let host = env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS missing");
        let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing");

        format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
    })
});

pub async fn populate_notes() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(&DATABASE_URL).await?;

    let sql = include_str!("../sql/populate_notes_table.sql");

    sqlx::query(sql).execute(&pool).await?;

    Ok(())
}

pub async fn clear_notes() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(&DATABASE_URL).await?;

    let sql = include_str!("../sql/clear_notes_table.sql");

    sqlx::query(sql).execute(&pool).await?;

    Ok(())
}
