use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use crate::app::constant::*;

pub struct AppError {
    pub error_code: AppErrorCode,
    pub error_details: Vec<ErrorEntry>,
}

impl AppError {
    pub fn new(app_error_code: AppErrorCode, error_details: ErrorEntry) -> Self {
        Self {
            error_code: app_error_code, // Default code, can be overridden later.
            error_details: vec![error_details],
        }
    }
}
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        // Do not disclose database-related internal specifics, except for debug builds.
        if cfg!(debug_assertions) {
            let error_code = match e {
                sqlx::Error::RowNotFound => AppErrorCode::ResourceNotFound,
                _ => AppErrorCode::InternalServerError,
            };
            Self::new(error_code, ErrorEntry::new(&e.to_string()))
        } else {
            // Build the entry with a trace id to find the exact error in the log when needed.
            let error_code = AppErrorCode::DatabaseError;
            let error_entry = ErrorEntry::new(&e.to_string()).trace_id();
            let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
            // The error must be logged here. Otherwise, we would lose it.
            tracing::error!("SQLx error: {}, trace id: {}", e.to_string(), trace_id);
            Self::new(error_code, error_entry)
        }
    }
}

//implement Display for AppError
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "AppError: {:?}, Errors: {:?}",
            self.error_code, self.error_details
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = StatusCode::UNAUTHORIZED;;
        status_code.into_response()
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
    pub const fn numeric_code(self) -> u32 {
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
            AppErrorCode::TransferDestinationAccountNotFound => E_TRANSFER_DESTINATION_ACCOUNT_NOT_FOUND,
            AppErrorCode::TransferAccountsAreSame => E_TRANSFER_ACCOUNTS_ARE_SAME,
            AppErrorCode::ResourceNotFound => E_RESOURCE_NOT_FOUND,
            AppErrorCode::ApiVersionError => E_API_VERSION_ERROR,
            AppErrorCode::DatabaseError => E_DATABASE_ERROR,
            AppErrorCode::RedisError => E_REDIS_ERROR,
        }
    }
}

impl Display for AppErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            serde_json::json!(self).as_str().unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ErrorEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn trace_id(mut self) -> Self {
        // Generate a new trace id.
        let mut trace_id = uuid::Uuid::new_v4().to_string();
        trace_id.retain(|c| c != '-');
        self.trace_id = Some(trace_id);
        self
    }
}
