use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::app::errors::*;
use crate::app::state::AppState;
use crate::rest::sessions::auth_utils;
use crate::rest::sessions::claim::{AccessClaims, Claimable, RefreshClaims, decode_token};

impl<S> FromRequestParts<S> for AccessClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_part(parts, state).await
    }
}

impl<S> FromRequestParts<S> for RefreshClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_part(parts, state).await
    }
}

async fn decode_token_from_request_part<S, T>(parts: &mut Parts, state: &S) -> Result<T, AppError>
where
    AppState: FromRef<S>,
    S: Send + Sync,
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug + Claimable + Sync + Send,
{
    // Extract the token from the authorization header.
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| {
            println!("Invalid authorization header");
            AppError::new(
                AppErrorCode::AuthenticationMissingCredentials,
                ErrorEntry::new("Invalid authorization header"),
            )
        })?;

    // Take the state from a reference.
    let state = Arc::new(AppState::from_ref(state));

    // Decode the token.
    let claims = decode_token::<T>(bearer.token(), &state.app_config)?;

    // Check for revoked tokens if enabled by configuration.
    if state.app_config.jwt_enable_revoked_tokens {
        auth_utils::validate_revoked(&claims, &state).await?
    }
    Ok(claims)
}
