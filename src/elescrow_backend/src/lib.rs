use candid::{candid_method, export_service, Principal};
use ic_cdk_macros::*;
use ic_cdk::api::time;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use ic_websocket_cdk::types::{
    OnCloseCallbackArgs, OnMessageCallbackArgs, OnOpenCallbackArgs,
    WsHandlers, WsInitParams,
};
use std::cell::RefCell;
use std::sync::Once;

pub use types::common::*;
pub use types::errors::*;
use crate::types::transaction::*;
use crate::types::user::*;
use crate::types::notification::{
    Notification,
    NotificationFilter, 
    NotificationStats
};

mod messaging;
mod types;
mod models;
mod services;
mod storage;
mod security;
mod utils;
mod api;

pub use messaging::*;
pub use types::*;

use services::{
    user_service::UserService,
    transaction_service::TransactionService,
    notification_service::NotificationService,
    balance_service::BalanceService,
};

use security::{
    audit::AuditLogger,
};

static STORAGE_INIT: Once = Once::new();
thread_local! {
    pub static USER_SERVICE: RefCell<UserService> = RefCell::new(UserService::new());
    pub static TRANSACTION_SERVICE: RefCell<TransactionService> = RefCell::new(TransactionService::new());
    pub static NOTIFICATION_SERVICE: RefCell<NotificationService> = RefCell::new(NotificationService::new());
    pub static BALANCE_SERVICE: RefCell<BalanceService> = RefCell::new(BalanceService::new());
    
    pub static SYSTEM_STATE: RefCell<SystemState> = RefCell::new(SystemState::default());
    pub static AUDIT_LOGGER: RefCell<AuditLogger> = RefCell::new(AuditLogger::with_defaults());

    static INIT_TIMESTAMP: RefCell<u64> = RefCell::new(0);
}

pub fn get_memory(id: u8) -> Memory {
    MEM.with(|m| m.borrow().get(MemoryId::new(id)))
}

fn get_total_memory_usage() -> u64 {
    messaging::get_message_count()
}

#[post_upgrade]
fn post_upgrade() {
    init();
}

fn ensure_storage_initialized() {
    STORAGE_INIT.call_once(|| {
        let _ = storage::stable_storage::StorageManager::instance();
        
        if let Err(e) = storage::memory::utils::validate_memory_regions() {
            ic_cdk::trap(&format!("Memory region validation failed: {}", e));
        }
        
        ic_cdk::println!("Storage subsystem initialized successfully");
    });
}

#[init]
#[candid_method(init)]
fn init() {
    ensure_storage_initialized();
    INIT_TIMESTAMP.with(|t| *t.borrow_mut() = time());
    ic_cdk::println!("Elescrow canister initialized at {}", time());

        let handlers = WsHandlers {
        on_open: Some(|args: OnOpenCallbackArgs| {
            messaging::on_client_open(args.client_principal);
        }),
        on_message: Some(|args: OnMessageCallbackArgs| {
            messaging::on_ws_message(args.client_principal, args.message);
        }),
        on_close: Some(|args: OnCloseCallbackArgs| {
            messaging::on_client_close(args.client_principal);
        }),
    };
    let params = WsInitParams {
        handlers,
        ..WsInitParams::default()
    };
    ic_websocket_cdk::init(params);
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Preparing for upgrade...");
}

#[post_upgrade]
fn post_upgrade() {
    init();
    INIT_TIMESTAMP.with(|t| *t.borrow_mut() = time());
    ic_cdk::println!("Elescrow canister upgraded at {}", time());
}

pub use api::user_api::*;

pub use api::transaction_api::*;

pub use api::notification_api::*;

pub use api::admin_api::*;

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