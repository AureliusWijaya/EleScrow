use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MemoryRegion {
    Users = 0,
    UserIndex = 1,
    Transactions = 2,
    TransactionIndex = 3,
    Balances = 4,
    BalanceHistory = 5,
    Notifications = 6,
    NotificationIndex = 7,
    RateLimits = 8,
    AuditLogs = 9,
    Sessions = 10,
    Configuration = 11,
    Statistics = 12,
    Reserved1 = 13,
    Reserved2 = 14,
    Reserved3 = 15,
}

impl From<MemoryRegion> for MemoryId {
    fn from(region: MemoryRegion) -> Self {
        MemoryId::new(region as u8)
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub fn get_memory(region: MemoryRegion) -> Memory {
    MEMORY_MANAGER.with(|manager| {
        manager.borrow().get(region.into())
    })
}

#[derive(Clone, Debug)]
pub struct MemoryStats {
    pub total_allocated_pages: u64,
    pub total_used_pages: u64,
    pub total_allocated_bytes: u64,
    pub total_used_bytes: u64,
    pub regions: Vec<RegionStats>,
    pub page_size: u64,
}

#[derive(Clone, Debug)]
pub struct RegionStats {
    pub region: String,
    pub allocated_pages: u64,
    pub used_pages: u64,
    pub used_bytes: u64,
}

pub mod utils {
    use super::*;
    
    #[derive(Debug, PartialEq)]
    pub enum MemoryPressure {
        Low,
        Medium,
        High,
        Critical,
    }
    
    pub fn suggest_cleanup_actions(pressure: MemoryPressure) -> Vec<String> {
        match pressure {
            MemoryPressure::Low => vec![],
            MemoryPressure::Medium => vec![
                "Consider archiving old audit logs".to_string(),
                "Clean up expired sessions".to_string(),
            ],
            MemoryPressure::High => vec![
                "Archive old audit logs".to_string(),
                "Clean up expired sessions".to_string(),
                "Remove old notifications".to_string(),
                "Compact transaction history".to_string(),
            ],
            MemoryPressure::Critical => vec![
                "URGENT: Archive old data immediately".to_string(),
                "Consider upgrading canister storage".to_string(),
                "Implement data pruning strategies".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_region_conversion() {
        let region = MemoryRegion::Users;
        let memory_id: MemoryId = region.into();
        assert_eq!(memory_id, MemoryId::new(0));
    }
}