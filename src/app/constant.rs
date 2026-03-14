// Roles.
pub const USER_ROLE_ADMIN: &str = "admin";
pub const USER_ROLE_COMMON: &str = "common";
pub const USER_ROLE_GUEST: &str = "guest";

// Numeric error code constants (HTTP-style status codes)
pub const E_INTERNAL_SERVER_ERROR: u16 = 500;

pub const E_AUTH_WRONG_CREDENTIALS: u16 = 401;
pub const E_AUTH_MISSING_CREDENTIALS: u16 = 401;
pub const E_AUTH_TOKEN_CREATION_ERROR: u16 = 500;
pub const E_AUTH_INVALID_TOKEN: u16 = 401;
pub const E_AUTH_REVOKED_TOKENS_INACTIVE: u16 = 401;
pub const E_AUTH_FORBIDDEN: u16 = 403;

pub const E_USER_NOT_FOUND: u16 = 404;
pub const E_TRANSACTION_NOT_FOUND: u16 = 404;

pub const E_TRANSFER_INSUFFICIENT_FUNDS: u16 = 422;
pub const E_TRANSFER_SOURCE_ACCOUNT_NOT_FOUND: u16 = 404;
pub const E_TRANSFER_DESTINATION_ACCOUNT_NOT_FOUND: u16 = 404;
pub const E_TRANSFER_ACCOUNTS_ARE_SAME: u16 = 409;

pub const E_RESOURCE_NOT_FOUND: u16 = 404;
pub const E_API_VERSION_ERROR: u16 = 400;

pub const E_DATABASE_ERROR: u16 = 503;
pub const E_REDIS_ERROR: u16 = 503;
