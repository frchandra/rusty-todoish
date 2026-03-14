// Roles.
pub const USER_ROLE_ADMIN: &str = "admin";
pub const USER_ROLE_COMMON: &str = "common";
pub const USER_ROLE_GUEST: &str = "guest";

// Numeric error code constants
pub const E_INTERNAL_SERVER_ERROR: u32 = 1000;
pub const E_AUTH_WRONG_CREDENTIALS: u32 = 2000;
pub const E_AUTH_MISSING_CREDENTIALS: u32 = 2001;
pub const E_AUTH_TOKEN_CREATION_ERROR: u32 = 2002;
pub const E_AUTH_INVALID_TOKEN: u32 = 2003;
pub const E_AUTH_REVOKED_TOKENS_INACTIVE: u32 = 2004;
pub const E_AUTH_FORBIDDEN: u32 = 2005;

pub const E_USER_NOT_FOUND: u32 = 3000;
pub const E_TRANSACTION_NOT_FOUND: u32 = 4000;

pub const E_TRANSFER_INSUFFICIENT_FUNDS: u32 = 5000;
pub const E_TRANSFER_SOURCE_ACCOUNT_NOT_FOUND: u32 = 5001;
pub const E_TRANSFER_DESTINATION_ACCOUNT_NOT_FOUND: u32 = 5002;
pub const E_TRANSFER_ACCOUNTS_ARE_SAME: u32 = 5003;

pub const E_RESOURCE_NOT_FOUND: u32 = 6000;
pub const E_API_VERSION_ERROR: u32 = 6001;

pub const E_DATABASE_ERROR: u32 = 7000;
pub const E_REDIS_ERROR: u32 = 8000;
