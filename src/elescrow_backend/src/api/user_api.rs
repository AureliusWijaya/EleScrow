use candid::candid_method;
use ic_cdk_macros::{query, update};
use ic_cdk::api::msg_caller;

use crate::types::{
    errors::ApiError,
    user::*,
    common::PaginationParams
};

use crate::{USER_SERVICE};

#[update]
#[candid_method(update)]
pub async fn register_user(request: RegisterUserRequest) -> Result<User, ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().register(caller, request)
    })
}

#[query]
#[candid_method(query)]
pub fn get_current_user() -> Result<User, ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().get_user(caller)
    })
}

#[query]
#[candid_method(query)]
pub fn get_user_by_principal(principal: candid::Principal) -> Result<User, ApiError> {
    USER_SERVICE.with(|service| {
        service.borrow().get_user(principal)
    })
}

#[query]
#[candid_method(query)]
pub fn get_user_by_username(username: String) -> Result<User, ApiError> {
    USER_SERVICE.with(|service| {
        service.borrow().get_user_by_username(&username)
    })
}

#[update]
#[candid_method(update)]
pub fn update_profile(request: UpdateProfileRequest) -> Result<User, ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().update_profile(caller, request)
    })
}

#[update]
#[candid_method(update)]
pub fn update_notification_preferences(preferences: NotificationPreferences) -> Result<User, ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().update_notification_preferences(caller, preferences)
    })
}

#[update]
#[candid_method(update)]
pub fn update_security_settings(settings: SecuritySettings) -> Result<User, ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().update_security_settings(caller, settings)
    })
}

#[update]
#[candid_method(update)]
pub fn deactivate_account() -> Result<(), ApiError> {
    let caller = msg_caller();
    
    USER_SERVICE.with(|service| {
        service.borrow().deactivate_account(caller)
    })
}

#[query]
#[candid_method(query)]
pub fn search_users(
    params: UserSearchParams,
    pagination: PaginationParams,
) -> Result<Vec<User>, ApiError> {
    USER_SERVICE.with(|service| {
        service.borrow().search_users(params, pagination)
    })
}

#[query]
#[candid_method(query)]
pub fn is_username_available(username: String) -> bool {
    USER_SERVICE.with(|service| {
        service.borrow().get_user_by_username(&username).is_err()
    })
}