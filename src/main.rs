mod api;

use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let db = std::env::var("POSTGRES_DATABASE_NAME").unwrap();
        let user = std::env::var("POSTGRES_USERNAME").unwrap();
        let pass = std::env::var("POSTGRES_PASSWORD").unwrap();
        let host = std::env::var("POSTGRES_ADDRESS").unwrap();
        let port = std::env::var("POSTGRES_PORT").unwrap();

        format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
    });

    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {err}");
            std::process::exit(1);
        }
    };

    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => {
            println!("Database ping successful");
        }
        Err(err) => {
            eprintln!("Database ping failed: {err}");
            std::process::exit(1);
        }
    }


    //create router from the router module
    let app = api::routes::health_check_routes::routes();


    println!("Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}