use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db = env::var("POSTGRES_DATABASE_NAME").expect("POSTGRES_DATABASE_NAME missing");
            let user = env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME missing");
            let pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing");
            let host = env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS missing");
            let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing");

            format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
        });

        Self {
            database_url,
            bind_addr: "127.0.0.1:8080".to_string(),
        }
    }
}
