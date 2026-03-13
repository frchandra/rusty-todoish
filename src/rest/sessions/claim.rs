use serde::{Deserialize, Serialize};
use crate::rest::sessions::role;
use crate::rest::sessions::auth_utils::AuthError;

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
    fn validate_role_admin(&self) -> Result<(), AuthError>;
    fn get_sub(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    fn get_jti(&self) -> &str;
}

impl Claimable for AccessClaims {
    fn validate_role_admin(&self) -> Result<(), AuthError> {
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
    fn validate_role_admin(&self) -> Result<(), AuthError> {
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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum JwtTokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}