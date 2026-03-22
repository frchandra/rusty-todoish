use crate::app::config::AppConfig;
use crate::app::constant::*;
use crate::app::errors::AppError;
use crate::app::errors::*;
use crate::app::state::AppState;
use crate::models::user::UserModel;
use crate::rest::sessions::role;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::MutexGuard;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}
impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::AccessToken,
            1 => Self::RefreshToken,
            _ => Self::UnknownToken,
        }
    }
}

pub trait Claimable {
    fn validate_role_admin(&self) -> Result<(), AppError>;
    fn validate_role_admin_or_user(&self) -> Result<(), AppError>;
    fn get_sub(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    fn get_jti(&self) -> &str;
}

// [JWT Claims]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    /// Subject.
    pub sub: String,
    /// JWT ID.
    pub jti: String,
    /// Issued time.
    pub iat: usize,
    /// Expiration time.
    pub exp: usize,
    /// Token type.
    pub typ: u8,
    /// Role.
    pub role: String,
}

impl Claimable for AccessToken {
    fn validate_role_admin(&self) -> Result<(), AppError> {
        role::is_role_admin(&self.role)
    }
    fn validate_role_admin_or_user(&self) -> Result<(), AppError> {
        role::is_role_admin_or_user(&self.role)
    }
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    /// Subject.
    pub sub: String,
    /// JWT ID.
    pub jti: String,
    /// Issued time.
    pub iat: usize,
    /// Expiration time.
    pub exp: usize,
    /// Reference to paired access token,
    pub prf: String,
    /// Expiration time of paired access token,
    pub pex: usize,
    /// Token type.
    pub typ: u8,
    /// Role.
    pub role: String,
}

impl Claimable for RefreshToken {
    fn validate_role_admin(&self) -> Result<(), AppError> {
        role::is_role_admin(&self.role)
    }

    fn validate_role_admin_or_user(&self) -> Result<(), AppError> {
        role::is_role_admin_or_user(&self.role)
    }

    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn decode_token<T: for<'de> serde::Deserialize<'de>>(
    token: &str,
    config: &AppConfig,
) -> Result<T, AppError> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.leeway = config.jwt_validation_leeway_seconds as u64;
    let token_data = jsonwebtoken::decode::<T>(token, &config.jwt_keys.decoding, &validation)
        .map_err(|_| {
            tracing::error!("Invalid token: {}", token);
            AppError::new(
                AppErrorCode::AuthenticationWrongCredentials,
                "Invalid token",
            )
        })?;

    Ok(token_data.claims)
}

pub fn generate_tokens(user: UserModel, config: &AppConfig) -> TokenPair {
    let time_now = chrono::Utc::now();
    let iat = time_now.timestamp() as usize;
    let sub = user.id.to_string();

    let access_token_id = Uuid::new_v4().to_string();
    let refresh_token_id = Uuid::new_v4().to_string();
    let access_token_exp = (time_now
        + chrono::Duration::seconds(config.jwt_expire_access_token_seconds))
    .timestamp() as usize;

    let access_claims = AccessToken {
        sub: sub.clone(),
        jti: access_token_id.clone(),
        iat,
        exp: access_token_exp,
        typ: TokenType::AccessToken as u8,
        role: user.role.clone(),
    };

    let refresh_claims = RefreshToken {
        sub,
        jti: refresh_token_id,
        iat,
        exp: (time_now + chrono::Duration::seconds(config.jwt_expire_refresh_token_seconds))
            .timestamp() as usize,
        prf: access_token_id,
        pex: access_token_exp,
        typ: TokenType::RefreshToken as u8,
        role: user.role,
    };

    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    let refresh_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &refresh_claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap();

    TokenPair {
        access_token,
        refresh_token,
    }
}

pub async fn validate_revoked<T: std::fmt::Debug + Claimable + Sync + Send>(
    claims: &T,
    state: &AppState,
) -> Result<(), AppError> {
    let revoked = is_revoked(claims, state).await?;
    // let revoked = false; // Placeholder for actual revoked token check.
    if revoked {
        Err(AppError::new(
            AppErrorCode::ApiVersionError,
            "Token has been revoked",
        ))?;
    }
    Ok(())
}

pub fn validate_token_type(claims: &RefreshToken, expected_type: TokenType) -> bool {
    if claims.typ == expected_type as u8 {
        true
    } else {
        tracing::error!(
            "Invalid token type. Expected {:?}, Found {:?}",
            expected_type,
            TokenType::from(claims.typ),
        );
        false
    }
}

pub async fn revoke_global(state: &AppState) -> RedisResult<()> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    tracing::debug!("setting a timestamp for global revoke: {}", timestamp_now);
    state
        .redis_connection
        .lock()
        .await
        .set(JWT_REDIS_REVOKE_GLOBAL_BEFORE_KEY, timestamp_now)
        .await
}

async fn is_global_revoked<T: Claimable + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check in global revoke.
    let opt_exp: Option<String> = redis.get(JWT_REDIS_REVOKE_GLOBAL_BEFORE_KEY).await?;
    if let Some(exp) = opt_exp {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn is_user_revoked<T: Claimable + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check in user revoke.
    let user_id = claims.get_sub();
    let opt_exp: Option<String> = redis
        .hget(JWT_REDIS_REVOKE_USER_BEFORE_KEY, user_id)
        .await?;
    if let Some(exp) = opt_exp {
        let global_exp = exp.parse::<usize>().unwrap();
        if global_exp >= claims.get_iat() {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn is_token_revoked<T: Claimable + Sync + Send>(
    claims: &T,
    redis: &mut MutexGuard<'_, MultiplexedConnection>,
) -> RedisResult<bool> {
    // Check the token in revoked list.
    redis
        .hexists(JWT_REDIS_REVOKED_TOKENS_KEY, claims.get_jti())
        .await
}

pub async fn revoke_user_tokens(user_id: &str, state: &AppState) -> RedisResult<()> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;
    tracing::debug!(
        "adding a timestamp for user revoke, user:{}, timestamp: {}",
        user_id,
        timestamp_now
    );
    state
        .redis_connection
        .lock()
        .await
        .hset(JWT_REDIS_REVOKE_USER_BEFORE_KEY, user_id, timestamp_now)
        .await
}

pub async fn revoke_refresh_token(claims: &RefreshToken, state: &AppState) -> RedisResult<()> {
    // Adds refresh token and its paired access token into revoked list in Redis.
    // Tokens are tracked by JWT ID that handles the cases of reusing lost tokens and multi-device scenarios.

    let list_to_revoke = vec![&claims.jti, &claims.prf];
    tracing::debug!("adding jwt tokens into revoked list: {:#?}", list_to_revoke);

    let mut redis = state.redis_connection.lock().await;
    for claims_jti in list_to_revoke {
        let _: () = redis
            .hset(JWT_REDIS_REVOKED_TOKENS_KEY, claims_jti, claims.exp)
            .await?;
    }

    if tracing::enabled!(tracing::Level::TRACE) {
        log_revoked_tokens_count(&mut redis).await;
    }
    drop(redis);

    Ok(())
}

pub async fn is_revoked<T: std::fmt::Debug + Claimable + Send + Sync>(
    claims: &T,
    state: &AppState,
) -> RedisResult<bool> {
    let mut redis = state.redis_connection.lock().await;

    let global_revoked = is_global_revoked(claims, &mut redis).await?;
    if global_revoked {
        tracing::error!("Access denied (globally revoked): {:#?}", claims);
        return Ok(true);
    }

    let user_revoked = is_user_revoked(claims, &mut redis).await?;
    if user_revoked {
        tracing::error!("Access denied (user revoked): {:#?}", claims);
        return Ok(true);
    }

    let token_revoked = is_token_revoked(claims, &mut redis).await?;
    if token_revoked {
        tracing::error!("Access denied (token revoked): {:#?}", claims);
        return Ok(true);
    }

    drop(redis);
    Ok(false)
}

pub async fn cleanup_expired(state: &AppState) -> RedisResult<usize> {
    let timestamp_now = chrono::Utc::now().timestamp() as usize;

    let mut redis = state.redis_connection.lock().await;

    let revoked_tokens: HashMap<String, String> =
        redis.hgetall(JWT_REDIS_REVOKED_TOKENS_KEY).await?;

    let mut deleted = 0;
    for (key, exp) in revoked_tokens {
        match exp.parse::<usize>() {
            Ok(timestamp_exp) => {
                if timestamp_now > timestamp_exp {
                    // Workaround for https://github.com/redis-rs/redis-rs/issues/1322
                    let _: () = redis.hdel(JWT_REDIS_REVOKED_TOKENS_KEY, key).await?;
                    deleted += 1;
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }

    if tracing::enabled!(tracing::Level::TRACE) {
        log_revoked_tokens_count(&mut redis).await;
    }
    drop(redis);

    Ok(deleted)
}

pub async fn log_revoked_tokens_count(redis: &mut MultiplexedConnection) {
    let redis_result: RedisResult<usize> = redis.hlen(JWT_REDIS_REVOKED_TOKENS_KEY).await;
    match redis_result {
        Ok(revoked_tokens_count) => {
            println!(
                "REDIS: count of revoked jwt tokens: {}",
                revoked_tokens_count
            );
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub async fn log_revoked_tokens(redis: &mut MultiplexedConnection) {
    let redis_result: RedisResult<HashMap<String, String>> =
        redis.hgetall(JWT_REDIS_REVOKED_TOKENS_KEY).await;

    match redis_result {
        Ok(revoked_tokens) => {
            println!("REDIS: list of revoked jwt tokens: {:#?}", revoked_tokens);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
