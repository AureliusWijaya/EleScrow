use candid::candid_method;
use ic_cdk_macros::{query, update};
use ic_cdk::api::msg_caller;

use crate::types::{
    errors::ApiError,
    notification::{Notification, NotificationFilter, NotificationStats},
    common::{PaginationParams, ListResponse},
};

use crate::types::user::NotificationPreferences;
use crate::NOTIFICATION_SERVICE;

#[query]
#[candid_method(query)]
pub fn get_notifications(
    filter: Option<NotificationFilter>,
    pagination: PaginationParams,
) -> Result<ListResponse<Notification>, ApiError> {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().get_user_notifications(caller, filter, pagination)
    })
}

#[query]
#[candid_method(query)]
pub fn get_notification_preferences() -> Result<NotificationPreferences, ApiError> {
    let caller = msg_caller();
    
    crate::USER_SERVICE.with(|service| {
        let user = service.borrow().get_user(caller)?;
        Ok(user.notification_preferences)
    })
}

#[query]
#[candid_method(query)]
pub fn get_notification_stats() -> NotificationStats {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        let service = service.borrow();
        
        let total = service.get_user_notifications(caller, None, PaginationParams::new(Some(0), Some(1)))
            .map(|r| r.total)
            .unwrap_or(0);
        
        let unread = service.get_unread_count(caller);
        
        NotificationStats {
            total_notifications: total,
            unread_count: unread,
            read_count: total - unread,
            archived_count: 0, // Would need separate tracking
        }
    })
}

#[update]
#[candid_method(update)]
pub fn cleanup_expired_notifications() -> Result<u64, ApiError> {
    NOTIFICATION_SERVICE.with(|service| {
        Ok(service.borrow().cleanup_expired())
    })
}

#[query]
#[candid_method(query)]
pub fn get_unread_notifications(
    pagination: PaginationParams,
) -> Result<ListResponse<Notification>, ApiError> {
    let caller = msg_caller();
    
    let filter = NotificationFilter {
        unread_only: Some(true),
        priority: None,
        notification_type: None,
        category: None,
        date_range: None,
        has_actions: None,
    };
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().get_user_notifications(caller, Some(filter), pagination)
    })
}

#[query]
#[candid_method(query)]
pub fn get_unread_count() -> u64 {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().get_unread_count(caller)
    })
}

#[query]
#[candid_method(query)]
pub fn get_notification(notification_id: u64) -> Result<Notification, ApiError> {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        let notification = service.borrow().get_notification_model(notification_id)?;
        
        // Verify ownership
        if notification.recipient != caller {
            return Err(ApiError::Unauthorized {
                reason: "Cannot access another user's notification".to_string(),
            });
        }
        
        Ok(notification.into())
    })
}

#[update]
#[candid_method(update)]
pub fn mark_notification_read(notification_id: u64) -> Result<Notification, ApiError> {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().mark_as_read(notification_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn mark_all_notifications_read() -> Result<u64, ApiError> {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().mark_all_as_read(caller)
    })
}

#[update]
#[candid_method(update)]
pub fn archive_notification(notification_id: u64) -> Result<Notification, ApiError> {
    let caller = msg_caller();
    
    NOTIFICATION_SERVICE.with(|service| {
        service.borrow().archive(notification_id, caller)
    })
}
