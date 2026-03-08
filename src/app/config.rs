use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub service_version: String,
    pub database_url: String,
    pub redis_url: String,
    pub service_url: String,

    // jwt configuration
    pub jwt_secret: String,
    pub jwt_expire_access_token_seconds: i64,
    pub jwt_expire_refresh_token_seconds: i64,
    pub jwt_validation_leeway_seconds: i64,
    pub jwt_enable_revoked_tokens: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        // service configuration
        let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "SERVICE NAME missing".to_string());
        let service_version = env::var("SERVICE_VERSION").unwrap_or_else(|_| "SERVICE VERSION missing".to_string());

        // database configuration
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db = env::var("POSTGRES_DATABASE_NAME").expect("POSTGRES_DATABASE_NAME missing");
            let user = env::var("POSTGRES_USERNAME").expect("POSTGRES_USERNAME missing");
            let pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing");
            let host = env::var("POSTGRES_ADDRESS").expect("POSTGRES_ADDRESS missing");
            let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing");

            format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db)
        });

        // redis configuration
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| {
            let host = env::var("REDIS_HOST").expect("REDIS_HOST missing");
            let port = env::var("REDIS_PORT").expect("REDIS_PORT missing");
            format!("redis://{}:{}", host, port)
        });

        // rest api configuration
        let service_url = {
            let ip = env::var("SERVICE_HOST").expect("BIND_ADDRESS missing");
            let port = env::var("SERVICE_PORT").expect("BIND_PORT missing");
            format!("{}:{}", ip, port)
        };

        // jwt configuration
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");
        let jwt_expire_access_token_seconds = env::var("JWT_EXPIRE_ACCESS_TOKEN_SECONDS")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRE_ACCESS_TOKEN_SECONDS must be a u64");
        let jwt_expire_refresh_token_seconds = env::var("JWT_EXPIRE_REFRESH_TOKEN_SECONDS")
            .unwrap_or_else(|_| "7776000".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRE_REFRESH_TOKEN_SECONDS must be a u64");
        let jwt_validation_leeway_seconds = env::var("JWT_VALIDATION_LEEWAY_SECONDS")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<i64>()
            .expect("JWT_VALIDATION_LEEWAY_SECONDS must be a u64");
        let jwt_enable_revoked_tokens = env::var("JWT_ENABLE_REVOKED_TOKENS")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("JWT_ENABLE_REVOKED_TOKENS must be a bool");

        Self {
            service_name,
            service_version,
            database_url,
            redis_url,
            service_url,
            jwt_secret,
            jwt_expire_access_token_seconds,
            jwt_expire_refresh_token_seconds,
            jwt_validation_leeway_seconds,
            jwt_enable_revoked_tokens,
        }
    }
}
