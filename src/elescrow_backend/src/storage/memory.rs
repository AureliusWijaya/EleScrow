use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, Memory as MemoryTrait};
use std::cell::RefCell;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    UserTransactionsData = 16,
    Messages = 17,
    ConversationIndex = 18,
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

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct MemoryStats {
    pub total_allocated_pages: u64,
    pub total_used_pages: u64,
    pub total_allocated_bytes: u64,
    pub total_used_bytes: u64,
    pub regions: Vec<RegionStats>,
    pub page_size: u64,
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct RegionStats {
    pub region: String,
    pub allocated_pages: u64,
    pub used_pages: u64,
    pub used_bytes: u64,
}

pub mod utils {
    use super::*;
    
    #[derive(Debug, PartialEq, candid::CandidType, serde::Serialize, serde::Deserialize)]
    pub enum MemoryPressure {
        Low,
        Medium,
        High,
        Critical,
    }
    
    pub fn validate_memory_regions() -> Result<(), String> {
        // Ensure no duplicate memory region IDs
        let mut used_ids = std::collections::HashSet::new();
        
        let regions = [
            MemoryRegion::Users,
            MemoryRegion::UserIndex,
            MemoryRegion::Transactions,
            MemoryRegion::TransactionIndex,
            MemoryRegion::Balances,
            MemoryRegion::BalanceHistory,
            MemoryRegion::Notifications,
            MemoryRegion::NotificationIndex,
            MemoryRegion::RateLimits,
            MemoryRegion::AuditLogs,
            MemoryRegion::Sessions,
            MemoryRegion::Configuration,
            MemoryRegion::Statistics,
            MemoryRegion::Reserved1,
            MemoryRegion::Reserved2,
            MemoryRegion::Reserved3,
            MemoryRegion::UserTransactionsData,
        ];
        
        for region in regions.iter() {
            let id = *region as u8;
            if used_ids.contains(&id) {
                return Err(format!("Duplicate memory region ID: {}", id));
            }
            used_ids.insert(id);
        }
        
        Ok(())
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
        
        let region = MemoryRegion::UserTransactionsData;
        let memory_id: MemoryId = region.into();
        assert_eq!(memory_id, MemoryId::new(16));
    }
    
    #[test]
    fn test_memory_region_uniqueness() {
        utils::validate_memory_regions().expect("Memory regions should be unique");
    }
}