use serde::{Deserialize, Serialize};
use crate::app::errors::AppError;
use crate::rest::sessions::role;
// use crate::rest::sessions::auth_utils::AuthError;
use crate::app::config::AppConfig;
use crate::app::errors::*;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum JwtTokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}
impl From<u8> for JwtTokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::AccessToken,
            1 => Self::RefreshToken,
            _ => Self::UnknownToken,
        }
    }
}


// [JWT Claims]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
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

pub trait Claimable {
    fn validate_role_admin(&self) -> Result<(), AppError>;
    fn get_sub(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    fn get_jti(&self) -> &str;
}

impl Claimable for AccessClaims {
    fn validate_role_admin(&self) -> Result<(), AppError> {
        role::is_role_admin(&self.role)
    }
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}

impl Claimable for RefreshClaims {
    fn validate_role_admin(&self) -> Result<(), AppError> {
        role::is_role_admin(&self.role)
    }
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
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
                ErrorEntry::new("Invalid token"),
            )
        })?;

    Ok(token_data.claims)
}
