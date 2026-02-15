use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub service_version: String,
    pub database_url: String,
    pub service_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "SERVICE NAME missing".to_string());
        let service_version = env::var("SERVICE_VERSION").unwrap_or_else(|_| "SERVICE VERSION missing".to_string());

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db = env::var("POSTGRES_DATABASE_NAME").expect("POSTGRES_DATABASE_NAME missing");
            let user = env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME missing");
            let pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing");
            let host = env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS missing");
            let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing");

            format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
        });

        let service_url = {
            let ip = env::var("SERVICE_HOST").expect("BIND_ADDRESS missing");
            let port = env::var("SERVICE_PORT").expect("BIND_PORT missing");
            format!("{}:{}", ip, port)
        };

        Self {
            service_name,
            service_version,
            database_url,
            service_url,
        }
    }
}
