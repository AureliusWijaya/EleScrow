use candid::{CandidType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ApiError {
    NotFound { 
        resource: String 
    },
    AlreadyExists { 
        resource: String 
    },
    
    Unauthorized { 
        reason: String 
    },
    Forbidden { 
        action: String 
    },
    
    ValidationError { 
        field: String, 
        message: String 
    },
    BadRequest { 
        message: String 
    },
    
    InsufficientFunds { 
        available: u64, 
        required: u64 
    },
    TransactionError { 
        transaction_id: u64, 
        reason: String 
    },
    
    InvalidState { 
        current_state: String, 
        required_state: String 
    },
    Expired { 
        resource: String,
        expired_at: u64 
    },
    
    InternalError { 
        details: String 
    },
    ServiceUnavailable { 
        service: String 
    },
    
    RateLimited { 
        retry_after: u64 
    },
    
    AccountFrozen { 
        reason: String 
    },
    AccountNotVerified,
    SystemPaused { 
        reason: String 
    },
}

impl ApiError {
    pub fn to_string(&self) -> String {
        match self {
            ApiError::NotFound { resource } => 
                format!("{} not found", resource),
            
            ApiError::AlreadyExists { resource } => 
                format!("{} already exists", resource),
            
            ApiError::Unauthorized { reason } => 
                format!("Unauthorized: {}", reason),
            
            ApiError::Forbidden { action } => 
                format!("Forbidden: cannot {}", action),
            
            ApiError::ValidationError { field, message } => 
                format!("Validation error on {}: {}", field, message),
            
            ApiError::BadRequest { message } => 
                format!("Bad request: {}", message),
            
            ApiError::InsufficientFunds { available, required } => 
                format!("Insufficient funds: available {}, required {}", available, required),
            
            ApiError::TransactionError { transaction_id, reason } => 
                format!("Transaction {} error: {}", transaction_id, reason),
            
            ApiError::InvalidState { current_state, required_state } => 
                format!("Invalid state: current '{}', required '{}'", current_state, required_state),
            
            ApiError::Expired { resource, expired_at } => 
                format!("{} expired at {}", resource, expired_at),
            
            ApiError::InternalError { details } => 
                format!("Internal error: {}", details),
            
            ApiError::ServiceUnavailable { service } => 
                format!("{} service is unavailable", service),
            
            ApiError::RateLimited { retry_after } => 
                format!("Rate limited, retry after {} seconds", retry_after),
            
            ApiError::AccountFrozen { reason } => 
                format!("Account frozen: {}", reason),
            
            ApiError::AccountNotVerified => 
                "Account not verified".to_string(),

            ApiError::SystemPaused { reason } => 
                format!("System is paused: {}", reason),
        }
    }
    
    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::NotFound { .. } => "NOT_FOUND",
            ApiError::AlreadyExists { .. } => "ALREADY_EXISTS",
            ApiError::Unauthorized { .. } => "UNAUTHORIZED",
            ApiError::Forbidden { .. } => "FORBIDDEN",
            ApiError::ValidationError { .. } => "VALIDATION_ERROR",
            ApiError::BadRequest { .. } => "BAD_REQUEST",
            ApiError::InsufficientFunds { .. } => "INSUFFICIENT_FUNDS",
            ApiError::TransactionError { .. } => "TRANSACTION_ERROR",
            ApiError::InvalidState { .. } => "INVALID_STATE",
            ApiError::Expired { .. } => "EXPIRED",
            ApiError::InternalError { .. } => "INTERNAL_ERROR",
            ApiError::ServiceUnavailable { .. } => "SERVICE_UNAVAILABLE",
            ApiError::RateLimited { .. } => "RATE_LIMITED",
            ApiError::AccountFrozen { .. } => "ACCOUNT_FROZEN",
            ApiError::AccountNotVerified => "ACCOUNT_NOT_VERIFIED",
            ApiError::SystemPaused { .. } => "SYSTEM_PAUSED",
        }
    }
    
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ApiError::InternalError { .. } |
            ApiError::ServiceUnavailable { .. } |
            ApiError::RateLimited { .. }
        )
    }
}

impl From<String> for ApiError {
    fn from(error: String) -> Self {
        ApiError::InternalError { details: error }
    }
}

impl From<&str> for ApiError {
    fn from(error: &str) -> Self {
        ApiError::InternalError { details: error.to_string() }
    }
}

pub struct ValidationErrorBuilder {
    errors: Vec<(String, String)>,
}

impl ValidationErrorBuilder {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    
    pub fn add_error(mut self, field: &str, message: &str) -> Self {
        self.errors.push((field.to_string(), message.to_string()));
        self
    }
    
    pub fn build(self) -> Option<ApiError> {
        if self.errors.is_empty() {
            None
        } else if self.errors.len() == 1 {
            let (field, message) = self.errors.into_iter().next().unwrap();
            Some(ApiError::ValidationError { field, message })
        } else {
            let message = self.errors
                .into_iter()
                .map(|(field, msg)| format!("{}: {}", field, msg))
                .collect::<Vec<_>>()
                .join(", ");
            Some(ApiError::BadRequest { message })
        }
    }
}