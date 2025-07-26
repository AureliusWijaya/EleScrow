pub const MAX_BATCH_SIZE: usize = 100;
pub const MAX_QUERY_RESULTS: usize = 1000;
pub const DEFAULT_PAGE_SIZE: usize = 20;

pub const NANOS_PER_SECOND: u64 = 1_000_000_000;
pub const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SECOND;
pub const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;
pub const NANOS_PER_DAY: u64 = 24 * NANOS_PER_HOUR;
pub const NANOS_PER_WEEK: u64 = 7 * NANOS_PER_DAY;

pub const MIN_TRANSACTION_AMOUNT: u64 = 1_000;
pub const MAX_TRANSACTION_AMOUNT: u64 = 1_000_000_000_000;
pub const DEFAULT_TRANSACTION_FEE_BPS: u64 = 100;

pub const MAX_USERNAME_LENGTH: usize = 30;
pub const MIN_USERNAME_LENGTH: usize = 3;
pub const MAX_BIO_LENGTH: usize = 500;
pub const MAX_DISPLAY_NAME_LENGTH: usize = 100;

pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
pub const LOCKOUT_DURATION: u64 = 30 * NANOS_PER_MINUTE;
pub const SESSION_TIMEOUT: u64 = 24 * NANOS_PER_HOUR;
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;

pub const DEFAULT_RATE_LIMIT_REQUESTS: u32 = 100;
pub const DEFAULT_RATE_LIMIT_WINDOW: u64 = NANOS_PER_HOUR;
pub const DEFAULT_RATE_LIMIT_BLOCK_DURATION: u64 = 5 * NANOS_PER_MINUTE;

pub const MAX_NOTIFICATIONS_PER_USER: usize = 1000;
pub const NOTIFICATION_RETENTION_DAYS: u32 = 90;
pub const MAX_NOTIFICATION_TITLE_LENGTH: usize = 100;
pub const MAX_NOTIFICATION_MESSAGE_LENGTH: usize = 500;

pub const AUDIT_LOG_RETENTION_DAYS: u32 = 365;
pub const MAX_AUDIT_LOGS: u64 = 10_000_000;

pub const CYCLES_CREATION_FEE: u64 = 1_000_000_000_000;
pub const CYCLES_MINIMUM_BALANCE: u64 = 100_000_000_000;
pub const CYCLES_TOP_UP_AMOUNT: u64 = 500_000_000_000;

pub const MAX_HEAP_SIZE: u64 = 2 * 1024 * 1024 * 1024;
pub const MAX_STABLE_SIZE: u64 = 8 * 1024 * 1024 * 1024;

pub const ERR_UNAUTHORIZED: &str = "Unauthorized access";
pub const ERR_NOT_FOUND: &str = "Resource not found";
pub const ERR_INVALID_INPUT: &str = "Invalid input provided";
pub const ERR_INSUFFICIENT_FUNDS: &str = "Insufficient funds";
pub const ERR_RATE_LIMITED: &str = "Too many requests";

pub const MSG_REGISTRATION_SUCCESS: &str = "Registration successful";
pub const MSG_TRANSACTION_COMPLETE: &str = "Transaction completed successfully";
pub const MSG_PROFILE_UPDATED: &str = "Profile updated successfully";

pub const SYSTEM_PRINCIPAL: &str = "aaaaa-aa";
pub const TEST_PRINCIPAL: &str = "2vxsx-fae";

pub const ENABLE_DEBUG_LOGGING: bool = cfg!(debug_assertions);
pub const ENABLE_RATE_LIMITING: bool = true;
pub const ENABLE_AUDIT_LOGGING: bool = true;
pub const ENABLE_NOTIFICATIONS: bool = true;

pub const API_VERSION: &str = "1.0.0";
pub const MIN_SUPPORTED_CLIENT_VERSION: &str = "1.0.0";