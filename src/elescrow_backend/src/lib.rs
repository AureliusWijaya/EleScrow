use candid::{candid_method, export_service, Principal};
use ic_cdk_macros::*;
use ic_cdk::api::{caller, time};
use std::cell::RefCell;

mod types;
mod models;
mod services;
mod storage;
mod security;
mod utils;
mod api;

use types::common::*;
use types::errors::*;
use types::user::*;

use services::{
    user_service::UserService
};

use security::{
    audit::AuditLogger,
};


thread_local! {
    pub static USER_SERVICE: RefCell<UserService> = RefCell::new(UserService::new());
    pub static AUDIT_LOGGER: RefCell<AuditLogger> = RefCell::new(AuditLogger::with_defaults());
    
    // System state
    static INIT_TIMESTAMP: RefCell<u64> = RefCell::new(0);
}

#[init]
fn init() {
    INIT_TIMESTAMP.with(|t| *t.borrow_mut() = time());
    ic_cdk::println!("Elescrow canister initialized at {}", time());
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Preparing for upgrade...");
}

#[post_upgrade]
fn post_upgrade() {
    INIT_TIMESTAMP.with(|t| *t.borrow_mut() = time());
    ic_cdk::println!("Elescrow canister upgraded at {}", time());
}

#[query]
#[candid_method(query)]
fn health_check() -> HealthStatus {
    HealthStatus {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: time(),
        memory_usage: utils::helpers::get_memory_usage(),
        uptime: time() - INIT_TIMESTAMP.with(|t| *t.borrow()),
    }
}

#[query]
#[candid_method(query)]
fn get_canister_info() -> CanisterInfo {
    CanisterInfo {
        name: "Elescrow Backend".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        git_commit: option_env!("GIT_COMMIT_HASH").unwrap_or("unknown").to_string(),
        build_time: option_env!("BUILD_TIME").unwrap_or("unknown").to_string(),
        memory_usage: utils::helpers::get_memory_usage(),
        cycle_balance: ic_cdk::api::canister_balance(),
    }
}


// ===== User Management API =====
pub use api::user_api::{
    register_user,
    get_current_user,
    get_user_by_principal,
    get_user_by_username,
    update_profile,
    update_notification_preferences,
    update_security_settings,
    deactivate_account,
    search_users,
    is_username_available
};

export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct CanisterInfo {
    pub name: String,
    pub version: String,
    pub git_commit: String,
    pub build_time: String,
    pub memory_usage: MemoryUsage,
    pub cycle_balance: u64,
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
    
    #[test]
    fn test_health_check() {
        let health = health_check();
        assert_eq!(health.status, "healthy");
    }
}