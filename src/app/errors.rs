use crate::app::constant::*;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

pub struct AppError {
    pub error_code: AppErrorCode,
    pub error_details: String,
}

impl AppError {
    pub fn new(app_error_code: AppErrorCode, error_details: impl Into<String>) -> Self {
        Self {
            error_code: app_error_code,
            error_details: error_details.into(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        let error_code = match e {
            sqlx::Error::RowNotFound => AppErrorCode::ResourceNotFound,
            _ => AppErrorCode::InternalServerError,
        };
        Self::new(error_code, e.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(error: redis::RedisError) -> Self {
        Self {
            error_code: AppErrorCode::RedisError,
            error_details: error.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.error_code.http_status();
        let body = serde_json::json!({
            "code": self.error_code.numeric_code(),
            "error": self.error_code, // serialized as snake_case
            "details": self.error_details,
        });
        (status_code, Json(body)).into_response()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AppErrorCode {
    InternalServerError,
    AuthenticationWrongCredentials,
    AuthenticationMissingCredentials,
    AuthenticationTokenCreationError,
    AuthenticationInvalidToken,
    AuthenticationRevokedTokensInactive,
    AuthenticationForbidden,
    UserNotFound,
    TransactionNotFound,
    TransferInsufficientFunds,
    TransferSourceAccountNotFound,
    TransferDestinationAccountNotFound,
    TransferAccountsAreSame,
    ResourceNotFound,
    ApiVersionError,
    DatabaseError,
    RedisError,
}

impl AppErrorCode {
    pub const fn numeric_code(self) -> u16 {
        match self {
            AppErrorCode::InternalServerError => E_INTERNAL_SERVER_ERROR,
            AppErrorCode::AuthenticationWrongCredentials => E_AUTH_WRONG_CREDENTIALS,
            AppErrorCode::AuthenticationMissingCredentials => E_AUTH_MISSING_CREDENTIALS,
            AppErrorCode::AuthenticationTokenCreationError => E_AUTH_TOKEN_CREATION_ERROR,
            AppErrorCode::AuthenticationInvalidToken => E_AUTH_INVALID_TOKEN,
            AppErrorCode::AuthenticationRevokedTokensInactive => E_AUTH_REVOKED_TOKENS_INACTIVE,
            AppErrorCode::AuthenticationForbidden => E_AUTH_FORBIDDEN,
            AppErrorCode::UserNotFound => E_USER_NOT_FOUND,
            AppErrorCode::TransactionNotFound => E_TRANSACTION_NOT_FOUND,
            AppErrorCode::TransferInsufficientFunds => E_TRANSFER_INSUFFICIENT_FUNDS,
            AppErrorCode::TransferSourceAccountNotFound => E_TRANSFER_SOURCE_ACCOUNT_NOT_FOUND,
            AppErrorCode::TransferDestinationAccountNotFound => {
                E_TRANSFER_DESTINATION_ACCOUNT_NOT_FOUND
            }
            AppErrorCode::TransferAccountsAreSame => E_TRANSFER_ACCOUNTS_ARE_SAME,
            AppErrorCode::ResourceNotFound => E_RESOURCE_NOT_FOUND,
            AppErrorCode::ApiVersionError => E_API_VERSION_ERROR,
            AppErrorCode::DatabaseError => E_DATABASE_ERROR,
            AppErrorCode::RedisError => E_REDIS_ERROR,
        }
    }

    pub const fn http_status(self) -> StatusCode {
        match self.numeric_code() {
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            409 => StatusCode::CONFLICT,
            422 => StatusCode::UNPROCESSABLE_ENTITY,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            503 => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
