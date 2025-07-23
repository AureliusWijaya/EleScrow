use candid::{candid_method, export_service, Principal};
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use ic_websocket_cdk::types::{
    OnCloseCallbackArgs, OnMessageCallbackArgs, OnOpenCallbackArgs,
    WsHandlers, WsInitParams,
};
use std::cell::RefCell;

mod messaging;
mod types;

pub use messaging::*;
pub use types::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEM: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

pub fn get_memory(id: u8) -> Memory {
    MEM.with(|m| m.borrow().get(MemoryId::new(id)))
}

#[query]
#[candid_method(query)]
fn health_check() -> bool {
    true
}

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
    messaging::get_message_count()
}

#[init]
#[candid_method(init)]
fn init() {
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

#[post_upgrade]
fn post_upgrade() {
    init();
}

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