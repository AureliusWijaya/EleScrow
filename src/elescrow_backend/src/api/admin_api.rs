use candid::candid_method;
use ic_cdk_macros::{query, update};
use ic_cdk::api::msg_caller;

use crate::types::{
    errors::ApiError,
    common::{PaginationParams, AuditLog, AuditAction},
    user::UserSearchParams,
    transaction::{Transaction, DisputeResolution},
};

use crate::{USER_SERVICE, TRANSACTION_SERVICE, AUDIT_LOGGER};

fn ensure_admin(caller: candid::Principal) -> Result<(), ApiError> {
    let admin_principals = vec![
        "tdq4z-gz524-doqo7-nat24-nclox-v47yj-t5net-wnabe-nnr2g-fgl32-rqe",
    ];
    
    if !admin_principals.contains(&caller.to_text().as_str()) {
        return Err(ApiError::Unauthorized {
            reason: "Admin access required".to_string(),
        });
    }
    
    Ok(())
}

#[update]
#[candid_method(update)]
pub fn admin_freeze_account(
    user_principal: candid::Principal,
    reason: String,
) -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    USER_SERVICE.with(|service| {
        service.borrow().freeze_account(user_principal, reason, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_unfreeze_account(user_principal: candid::Principal) -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    USER_SERVICE.with(|service| {
        service.borrow().unfreeze_account(user_principal, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_verify_user(
    user_principal: candid::Principal,
    verification_level: crate::types::user::VerificationLevel,
) -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    USER_SERVICE.with(|service| {
        service.borrow().admin_update_verification_status(user_principal, verification_level, caller)?;
        Ok(())
    })
}

#[query]
#[candid_method(query)]
pub fn admin_search_users(
    params: UserSearchParams,
    pagination: PaginationParams,
) -> Result<Vec<crate::types::user::User>, ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    USER_SERVICE.with(|service| {
        service.borrow().search_users(params, pagination)
    })
}

#[query]
#[candid_method(query)]
pub fn admin_get_transaction(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().get_transaction(transaction_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_resolve_dispute(
    transaction_id: u64,
    resolution: DisputeResolution
) -> Result<Transaction, ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().resolve_dispute(transaction_id, resolution, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_reverse_transaction(
    transaction_id: u64,
    reason: String,
) -> Result<Transaction, ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().reverse_transaction(transaction_id, caller, reason)
    })
}

#[query]
#[candid_method(query)]
pub fn admin_get_audit_logs(
    pagination: PaginationParams,
) -> Result<Vec<AuditLog>, ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;
    
    AUDIT_LOGGER.with(|logger| {
        logger.borrow().get_logs(pagination)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_update_fee_percentage(new_fee_bps: u64) -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;

    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().update_fee_percentage(new_fee_bps, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn admin_pause_system(reason: String) -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;

    crate::SYSTEM_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.is_paused = true;
        s.reason = Some(reason.clone());
    });

    AUDIT_LOGGER.with(|log| {
        log.borrow().log(
            caller,
            crate::types::common::AuditAction::SystemPaused,
            "System",
            Some(format!("Reason: {}", reason)),
        );
    });

    Ok(())
}

#[update]
#[candid_method(update)]
pub fn admin_resume_system() -> Result<(), ApiError> {
    let caller = msg_caller();
    ensure_admin(caller)?;

    AUDIT_LOGGER.with(|log| {
        log.borrow().log(
            caller,
            AuditAction::SystemResumed,
            "System",
            None,
        );
    });

    Ok(())
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct AuditLogFilter {
    pub principal: Option<candid::Principal>,
    pub action: Option<crate::types::common::AuditAction>,
    pub date_range: Option<crate::types::common::TimeFilter>,
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceResult {
    pub expired_notifications_removed: u64,
    pub rate_limit_entries_cleaned: u64,
    pub audit_logs_archived: u64,
    pub memory_reclaimed_bytes: u64,
}