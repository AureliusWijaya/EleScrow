use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::api::time;
use crate::types::common::{AuditLog, AuditAction, PaginationParams};
use crate::types::errors::ApiError;
use crate::storage::{stable_storage::StableStorage, memory::MemoryRegion};
use ic_stable_structures::Storable;
use std::borrow::Cow;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct AuditConfig {
    pub retention_days: u32,
    pub max_entries: u64,
    pub enable_compression: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            retention_days: 90,
            max_entries: 1_000_000,
            enable_compression: true,
        }
    }
}

impl Storable for AuditLog {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 1024,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let serialized = serde_cbor::to_vec(self).expect("Failed to serialize AuditLog");
        Cow::Owned(serialized)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).expect("Failed to deserialize AuditLog")
    }
}

pub struct AuditLogger {
    storage: StableStorage<u64, AuditLog>,
    config: AuditConfig,
    next_id: RefCell<u64>,
}

impl AuditLogger {
    pub fn new(config: AuditConfig) -> Self {
        Self {
            storage: StableStorage::new(MemoryRegion::AuditLogs),
            config,
            next_id: RefCell::new(1),
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(AuditConfig::default())
    }
    
    pub fn log(
        &self,
        principal: Principal,
        action: AuditAction,
        resource: &str,
        details: Option<String>,
    ) -> u64 {
        let id = self.get_next_id();
        
        let log = AuditLog {
            id,
            timestamp: time(),
            principal,
            action,
            resource: resource.to_string(),
            details,
            ip_address: None,
            user_agent: None,
        };
        
        self.storage.insert(id, log);
        
        if id % 1000 == 0 {
            self.cleanup_old_entries();
        }
        
        id
    }
    
    pub fn log_with_context(
        &self,
        principal: Principal,
        action: AuditAction,
        resource: &str,
        details: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> u64 {
        let id = self.get_next_id();
        
        let log = AuditLog {
            id,
            timestamp: time(),
            principal,
            action,
            resource: resource.to_string(),
            details,
            ip_address,
            user_agent,
        };
        
        self.storage.insert(id, log);
        id
    }
    
    pub fn get_logs(&self, params: PaginationParams) -> Result<Vec<AuditLog>, ApiError> {
        params.validate()?;
        
        let logs = self.storage.paginate(params.offset, params.limit);
        Ok(logs.into_iter().map(|(_, log)| log).collect())
    }
    
    pub fn get_logs_by_principal(
        &self,
        principal: Principal,
        params: PaginationParams,
    ) -> Result<Vec<AuditLog>, ApiError> {
        params.validate()?;
        
        let filtered: Vec<AuditLog> = self.storage
            .filter(|_, log| log.principal == principal)
            .into_iter()
            .map(|(_, log)| log)
            .skip(params.offset as usize)
            .take(params.limit as usize)
            .collect();
        
        Ok(filtered)
    }
    
    pub fn get_logs_by_action(
        &self,
        action: AuditAction,
        params: PaginationParams,
    ) -> Result<Vec<AuditLog>, ApiError> {
        params.validate()?;
        
        let filtered: Vec<AuditLog> = self.storage
            .filter(|_, log| std::mem::discriminant(&log.action) == std::mem::discriminant(&action))
            .into_iter()
            .map(|(_, log)| log)
            .skip(params.offset as usize)
            .take(params.limit as usize)
            .collect();
        
        Ok(filtered)
    }
    
    pub fn get_logs_by_time_range(
        &self,
        start: u64,
        end: u64,
        params: PaginationParams,
    ) -> Result<Vec<AuditLog>, ApiError> {
        params.validate()?;
        
        if start > end {
            return Err(ApiError::ValidationError {
                field: "time_range".to_string(),
                message: "Start time must be before end time".to_string(),
            });
        }
        
        let filtered: Vec<AuditLog> = self.storage
            .filter(|_, log| log.timestamp >= start && log.timestamp <= end)
            .into_iter()
            .map(|(_, log)| log)
            .skip(params.offset as usize)
            .take(params.limit as usize)
            .collect();
        
        Ok(filtered)
    }
    
    pub fn search_logs(
        &self,
        query: &str,
        params: PaginationParams,
    ) -> Result<Vec<AuditLog>, ApiError> {
        params.validate()?;
        
        let query_lower = query.to_lowercase();
        
        let filtered: Vec<AuditLog> = self.storage
            .filter(|_, log| {
                log.resource.to_lowercase().contains(&query_lower) ||
                log.details.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
            })
            .into_iter()
            .map(|(_, log)| log)
            .skip(params.offset as usize)
            .take(params.limit as usize)
            .collect();
        
        Ok(filtered)
    }
    
    pub fn get_statistics(&self) -> AuditStatistics {
        let total_entries = self.storage.len();
        let now = time();
        let day_ago = now - (24 * 60 * 60 * 1_000_000_000);
        let week_ago = now - (7 * 24 * 60 * 60 * 1_000_000_000);
        
        let mut actions_count = std::collections::HashMap::new();
        let mut principals_count = std::collections::HashMap::new();
        let mut entries_last_day = 0;
        let mut entries_last_week = 0;
        
        for (_, log) in self.storage.entries() {
            let action_str = format!("{:?}", log.action);
            *actions_count.entry(action_str).or_insert(0) += 1;
            *principals_count.entry(log.principal).or_insert(0) += 1;
            if log.timestamp >= day_ago {
                entries_last_day += 1;
            }
            if log.timestamp >= week_ago {
                entries_last_week += 1;
            }
        }
        
        AuditStatistics {
            total_entries,
            entries_last_day,
            entries_last_week,
            unique_principals: principals_count.len() as u64,
            actions_breakdown: actions_count,
        }
    }
    
    pub fn export_logs(
        &self,
        start_id: u64,
        end_id: u64,
    ) -> Result<Vec<AuditLog>, ApiError> {
        if start_id > end_id {
            return Err(ApiError::ValidationError {
                field: "id_range".to_string(),
                message: "Start ID must be less than or equal to end ID".to_string(),
            });
        }
        
        let logs: Vec<AuditLog> = (start_id..=end_id)
            .filter_map(|id| self.storage.get(&id))
            .collect();
        
        Ok(logs)
    }
    
    fn get_next_id(&self) -> u64 {
        let mut id = self.next_id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    }
    
    fn cleanup_old_entries(&self) -> u64 {
        let retention_ns = self.config.retention_days as u64 * 24 * 60 * 60 * 1_000_000_000;
        let cutoff_time = time() - retention_ns;

        let to_remove: Vec<u64> = self.storage
            .filter(|_, log| log.timestamp < cutoff_time)
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        let count = to_remove.len() as u64;
        for id in to_remove {
            self.storage.remove(&id);
        }
        count
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct AuditStatistics {
    pub total_entries: u64,
    pub entries_last_day: u64,
    pub entries_last_week: u64,
    pub unique_principals: u64,
    pub actions_breakdown: std::collections::HashMap<String, u64>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct SuspiciousActivity {
    pub activity_type: String,
    pub principal: Principal,
    pub severity: Severity,
    pub details: String,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}
