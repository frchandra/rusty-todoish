use crate::app::config::AppConfig;
use crate::app::errors::{AppError, AppErrorCode};
use crate::app::state::AppState;
use crate::models::user::UserModel;
use crate::rest::sessions::claim::{AccessClaims, Claimable, JwtTokenType, RefreshClaims};
use uuid::Uuid;

pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_tokens(user: UserModel, config: &AppConfig) -> AuthTokens {
    let time_now = chrono::Utc::now();
    let iat = time_now.timestamp() as usize;
    let sub = user.id.to_string();

    let access_token_id = Uuid::new_v4().to_string();
    let refresh_token_id = Uuid::new_v4().to_string();
    let access_token_exp = (time_now
        + chrono::Duration::seconds(config.jwt_expire_access_token_seconds))
    .timestamp() as usize;

    let access_claims = AccessClaims {
        sub: sub.clone(),
        jti: access_token_id.clone(),
        iat,
        exp: access_token_exp,
        typ: JwtTokenType::AccessToken as u8,
        role: user.role.clone(),
    };

    let refresh_claims = RefreshClaims {
        sub,
        jti: refresh_token_id,
        iat,
        exp: (time_now + chrono::Duration::seconds(config.jwt_expire_refresh_token_seconds))
            .timestamp() as usize,
        prf: access_token_id,
        pex: access_token_exp,
        typ: JwtTokenType::RefreshToken as u8,
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

    AuthTokens {
        access_token,
        refresh_token,
    }
}

pub async fn validate_revoked<T: std::fmt::Debug + Claimable + Sync + Send>(
    claims: &T,
    state: &AppState,
) -> Result<(), AppError> {
    // let revoked = token_service::is_revoked(claims, state).await?;
    let revoked = false; // Placeholder for actual revoked token check.
    if revoked {
        Err(AppError::new(
            AppErrorCode::ApiVersionError,
            "Token has been revoked",
        ))?;
    }
    Ok(())
}
