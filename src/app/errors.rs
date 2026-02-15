use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

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

// impl From<sqlx::Error> for ErrorEntry {
//     fn from(e: sqlx::Error) -> Self {
//         // Do not disclose database-related internal specifics, except for debug builds.
//         /*        if cfg!(debug_assertions) {
//             let (code, kind) = match e {
//                 sqlx::Error::RowNotFound => (
//                     APIErrorCode::ResourceNotFound,
//                     APIErrorKind::ResourceNotFound,
//                 ),
//                 _ => (APIErrorCode::DatabaseError, APIErrorKind::DatabaseError),
//             };
//             Self::new(&e.to_string()).code(code).kind(kind).trace_id()
//         } else {
//             // Build the entry with a trace id to find the exact error in the log when needed.
//             let error_entry = Self::from(StatusCode::INTERNAL_SERVER_ERROR).trace_id();
//             let trace_id = error_entry.trace_id.as_deref().unwrap_or("");
//             // The error must be logged here. Otherwise, we would lose it.
//             tracing::error!("SQLx error: {}, trace id: {}", e.to_string(), trace_id);
//             error_entry
//         }*/
//         Self::new(&e.to_string()).trace_id()
//     }
// }
