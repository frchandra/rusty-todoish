use sqlx::PgPool;

pub async fn create_instance(database_url: &str) -> PgPool {
    let pool = PgPool::connect(database_url)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to connect to database: {err}");
            std::process::exit(1);
        });

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Database ping failed: {err}");
            std::process::exit(1);
        });

    println!("Database connection verified");
    pool
}
