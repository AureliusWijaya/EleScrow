use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<T, crate::types::errors::ApiError>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct PaginationParams {
    pub offset: u64,
    pub limit: u64,
}

impl PaginationParams {
    pub fn new(offset: Option<u64>, limit: Option<u64>) -> Self {
        Self {
            offset: offset.unwrap_or(0),
            limit: limit.unwrap_or(20).min(100),
        }
    }
    
    pub fn validate(&self) -> Result<(), crate::types::errors::ApiError> {
        use crate::types::errors::ApiError;
        
        if self.limit == 0 {
            return Err(ApiError::ValidationError {
                field: "limit".to_string(),
                message: "Limit must be greater than 0".to_string(),
            });
        }
        
        if self.limit > 100 {
            return Err(ApiError::ValidationError {
                field: "limit".to_string(),
                message: "Limit cannot exceed 100".to_string(),
            });
        }
        
        Ok(())
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub timestamp: u64,
    pub memory_usage: MemoryUsage,
    pub uptime: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub heap_size: u64,
    pub stable_size: u64,
    pub total_size: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_users: u64,
    pub active_users_24h: u64,
    pub total_transactions: u64,
    pub pending_transactions: u64,
    pub total_volume: u64,
    pub fees_collected: u64,
    pub avg_response_time_ms: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: u64,
    pub timestamp: u64,
    pub principal: Principal,
    pub action: AuditAction,
    pub resource: String,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum AuditAction {
    UserRegistered,
    UserUpdated,
    UserDeactivated,
    UserReactivated,
    LoginAttempt,
    LoginSuccess,
    LoginFailed,
    
    TransactionCreated,
    TransactionApproved,
    TransactionCompleted,
    TransactionCancelled,
    TransactionDisputed,
    TransactionRefunded,
    
    Deposit,
    Withdrawal,
    FundsLocked,
    FundsUnlocked,
    
    AdminAccess,
    ConfigurationChanged,
    AccountFrozen,
    AccountUnfrozen,
    
    RateLimitExceeded,
    SuspiciousActivity,
    ValidationFailed,
}

pub trait Timestamped {
    fn created_at(&self) -> u64;
    fn updated_at(&self) -> u64;
}
pub trait Identifiable {
    type Id;
    fn id(&self) -> Self::Id;
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TimeFilter {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl TimeFilter {
    pub fn is_in_range(&self, timestamp: u64) -> bool {
        let after_start = self.start.map_or(true, |start| timestamp >= start);
        let before_end = self.end.map_or(true, |end| timestamp <= end);
        after_start && before_end
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
    pub has_more: bool,
}

impl<T> ListResponse<T> {
    pub fn new(items: Vec<T>, total: u64, offset: u64, limit: u64) -> Self {
        let has_more = offset + (items.len() as u64) < total;
        Self {
            items,
            total,
            offset,
            limit,
            has_more,
        }
    }
}