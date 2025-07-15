use candid::{candid_method, export_service, Principal};
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use std::cell::RefCell;

// Import all modules
mod messaging;
mod types;

// Re-export public types and functions from modules
pub use messaging::*;
pub use types::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Global memory manager for all modules
thread_local! {
    static MEM: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

// Memory allocation for different modules
pub fn get_memory(id: u8) -> Memory {
    MEM.with(|m| m.borrow().get(MemoryId::new(id)))
}

// Health check for the entire canister
#[query]
#[candid_method(query)]
fn health_check() -> bool {
    true
}

// Get canister info
#[query]
#[candid_method(query)]
fn get_canister_info() -> CanisterInfo {
    CanisterInfo {
        name: "Elescrow Backend".to_string(),
        version: "0.1.0".to_string(),
        modules: vec![
            "messaging".to_string()
        ],
        total_memory_usage: get_total_memory_usage(),
    }
}

fn get_total_memory_usage() -> u64 {
    // Calculate total memory usage across all modules
    let message_count = get_message_count();
    message_count
}

// Generate the Candid interface for ALL modules
export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.join("candid");
        std::fs::create_dir_all(&dir).unwrap();
        write(dir.join("elescrow_backend.did"), export_candid()).expect("Write failed.");
    }
}