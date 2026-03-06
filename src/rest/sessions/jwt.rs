use serde::{Deserialize, Serialize};

use crate::application::{
    config::Config,
    // security::{auth::AuthError, roles},
};

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
    /// Roles.
    pub roles: String,
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
    /// Roles.
    pub roles: String,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum JwtTokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}