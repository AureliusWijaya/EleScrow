use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// Shared types across all modules
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct CanisterInfo {
    pub name: String,
    pub version: String,
    pub modules: Vec<String>,
    pub total_memory_usage: u64,
}

// Common result types
#[derive(CandidType, Deserialize)]
pub enum ApiResult<T> {
    Ok(T),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum BoolResult {
    Ok(bool),
    Err(String),
}

// User-related types (shared between messaging and transactions)
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct User {
    pub id: Principal,
    pub username: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

// Common pagination
#[derive(CandidType, Deserialize)]
pub struct PaginationParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(50),
        }
    }
}

// Common error types
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ErrorType {
    NotFound,
    Unauthorized,
    BadRequest,
    InternalError,
    ValidationError,
    InsufficientFunds,
    Expired,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ApiError {
    pub error_type: ErrorType,
    pub message: String,
    pub timestamp: u64,
}

impl ApiError {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
            timestamp: ic_cdk::api::time(),
        }
    }
    
    pub fn not_found(message: String) -> Self {
        Self::new(ErrorType::NotFound, message)
    }
    
    pub fn unauthorized(message: String) -> Self {
        Self::new(ErrorType::Unauthorized, message)
    }
    
    pub fn bad_request(message: String) -> Self {
        Self::new(ErrorType::BadRequest, message)
    }
    
    pub fn internal_error(message: String) -> Self {
        Self::new(ErrorType::InternalError, message)
    }
    
    pub fn validation_error(message: String) -> Self {
        Self::new(ErrorType::ValidationError, message)
    }
    
    pub fn insufficient_funds(message: String) -> Self {
        Self::new(ErrorType::InsufficientFunds, message)
    }
}

// Memory IDs for different modules
pub const MESSAGING_MEMORY_ID: u8 = 0;
pub const TRANSACTIONS_MEMORY_ID: u8 = 1;
pub const ESCROWS_MEMORY_ID: u8 = 2;
pub const BALANCES_MEMORY_ID: u8 = 3;
pub const USERS_MEMORY_ID: u8 = 4;

// Common constants
pub const MAX_TEXT_BYTES: u32 = 1000;
pub const MAX_PRINCIPAL_BYTES: u32 = 29;
pub const MAX_METADATA_BYTES: u32 = 2000;