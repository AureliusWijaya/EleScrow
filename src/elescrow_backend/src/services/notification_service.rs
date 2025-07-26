use candid::Principal;
use ic_cdk::api::time;
use std::cell::RefCell;

use crate::types::{
    errors::ApiError,
    notification::*,
    common::{PaginationParams, ListResponse},
};

use crate::models::notification::NotificationModel;
use crate::storage::{
    stable_storage::{StableStorage, IndexedStorage},
    memory::MemoryRegion,
};

pub struct NotificationService {
    notifications: StableStorage<u64, NotificationModel>,
    user_notifications: IndexedStorage<u64, NotificationModel, Principal>,
    
    next_id: RefCell<u64>,
    
    max_notifications_per_user: usize,
    default_expiry: u64,
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            notifications: StableStorage::new(MemoryRegion::Notifications),
            user_notifications: IndexedStorage::new(
                MemoryRegion::Notifications,
                MemoryRegion::NotificationIndex,
            ),
            next_id: RefCell::new(1),
            max_notifications_per_user: 1000,
            default_expiry: 30 * 24 * 60 * 60 * 1_000_000_000, 
        }
    }
    
    pub fn create(
        &self,
        recipient: Principal,
        notification_type: NotificationType,
        title: String,
        message: String,
        priority: NotificationPriority,
        related_resource: Option<RelatedResource>,
        actions: Vec<NotificationAction>,
    ) -> Result<Notification, ApiError> {
        let user_count = self.count_user_notifications(recipient);
        if user_count >= self.max_notifications_per_user {
            self.auto_archive_old_notifications(recipient);
        }
        
        let id = self.get_next_id();
        let now = time();
        
        let notification_model = NotificationModel {
            id,
            recipient,
            notification_type: notification_type.clone(),
            title,
            message,
            priority,
            category: Self::get_category_for_type(&notification_type),
            related_resource,
            actions,
            is_read: false,
            is_archived: false,
            created_at: now,
            read_at: None,
            expires_at: Some(now + self.default_expiry),
            delivery_status: DeliveryStatus::default(),
        };
        
        self.notifications.insert(id, notification_model.clone());
        self.user_notifications.insert_indexed(id, notification_model.clone(), recipient);
        
        Ok(notification_model.into())
    }
    
    pub fn create_transaction_notification(
        &self,
        recipient: Principal,
        transaction_id: u64,
        message: &str,
    ) -> Result<Notification, ApiError> {
        self.create(
            recipient,
            NotificationType::TransactionReceived,
            "Transaction Update".to_string(),
            message.to_string(),
            NotificationPriority::High,
            Some(RelatedResource::Transaction(transaction_id)),
            vec![
                NotificationAction {
                    id: "view".to_string(),
                    label: "View Transaction".to_string(),
                    action_type: ActionType::Navigate {
                        url: format!("/transactions/{}", transaction_id),
                    },
                    style: ActionStyle::Primary,
                    confirmation_required: false,
                },
            ],
        )
    }
    
    pub fn get_user_notifications(
        &self,
        user: Principal,
        filter: Option<NotificationFilter>,
        pagination: PaginationParams,
    ) -> Result<ListResponse<Notification>, ApiError> {
        pagination.validate()?;
        
        let mut notifications: Vec<NotificationModel> = self.notifications
            .filter(|_, n| {
                if n.recipient != user {
                    return false;
                }
                
                if let Some(ref f) = filter {
                    if let Some(unread_only) = f.unread_only {
                        if unread_only && n.is_read {
                            return false;
                        }
                    }
                    
                    if let Some(ref priorities) = f.priority {
                        if !priorities.iter().any(|p| p == &n.priority) {
                            return false;
                        }
                    }
                    
                    if let Some(ref categories) = f.category {
                        if !categories.iter().any(|c| c == &n.category) {
                            return false;
                        }
                    }
                }
                
                if let Some(expires_at) = n.expires_at {
                    if time() > expires_at {
                        return false;
                    }
                }
                
                !n.is_archived
            })
            .into_iter()
            .map(|(_, n)| n)
            .collect();
        
        notifications.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (NotificationPriority::Critical, NotificationPriority::Critical) => b.created_at.cmp(&a.created_at),
                (NotificationPriority::Critical, _) => std::cmp::Ordering::Less,
                (_, NotificationPriority::Critical) => std::cmp::Ordering::Greater,
                (NotificationPriority::Urgent, NotificationPriority::Urgent) => b.created_at.cmp(&a.created_at),
                (NotificationPriority::Urgent, _) => std::cmp::Ordering::Less,
                (_, NotificationPriority::Urgent) => std::cmp::Ordering::Greater,
                _ => b.created_at.cmp(&a.created_at),
            }
        });
        
        let total = notifications.len() as u64;
        let items: Vec<Notification> = notifications
            .into_iter()
            .skip(pagination.offset as usize)
            .take(pagination.limit as usize)
            .map(|n| n.into())
            .collect();
        
        Ok(ListResponse::new(items, total, pagination.offset, pagination.limit))
    }
    
    pub fn mark_as_read(&self, id: u64, user: Principal) -> Result<Notification, ApiError> {
        let mut notification = self.get_notification_model(id)?;
        
        if notification.recipient != user {
            return Err(ApiError::Unauthorized {
                reason: "Cannot read another user's notification".to_string(),
            });
        }
        
        if notification.is_read {
            return Ok(notification.into());
        }
        
        notification.is_read = true;
        notification.read_at = Some(time());
        notification.delivery_status.in_app = DeliveryState::Delivered {
            delivered_at: time(),
        };
        
        self.notifications.insert(id, notification.clone());
        
        Ok(notification.into())
    }
    
    pub fn mark_all_as_read(&self, user: Principal) -> Result<u64, ApiError> {
        let unread_notifications: Vec<(u64, NotificationModel)> = self.notifications
            .filter(|_, n| n.recipient == user && !n.is_read && !n.is_archived)
            .into_iter()
            .collect();
        
        let count = unread_notifications.len() as u64;
        let now = time();
        
        for (id, mut notification) in unread_notifications {
            notification.is_read = true;
            notification.read_at = Some(now);
            self.notifications.insert(id, notification);
        }
        
        Ok(count)
    }
    
    pub fn archive(&self, id: u64, user: Principal) -> Result<Notification, ApiError> {
        let mut notification = self.get_notification_model(id)?;
        
        if notification.recipient != user {
            return Err(ApiError::Unauthorized {
                reason: "Cannot archive another user's notification".to_string(),
            });
        }
        
        notification.is_archived = true;
        self.notifications.insert(id, notification.clone());
        
        Ok(notification.into())
    }
    
    pub fn delete(&self, id: u64, user: Principal) -> Result<(), ApiError> {
        let notification = self.get_notification_model(id)?;
        
        if notification.recipient != user {
            return Err(ApiError::Unauthorized {
                reason: "Cannot delete another user's notification".to_string(),
            });
        }
        
        self.notifications.remove(&id);
        self.user_notifications.remove_by_index(&user);
        
        Ok(())
    }

    pub fn get_unread_count(&self, user: Principal) -> u64 {
        self.notifications
            .filter(|_, n| n.recipient == user && !n.is_read && !n.is_archived)
            .len() as u64
    }
    
    pub fn cleanup_expired(&self) -> u64 {
        let now = time();
        let expired: Vec<u64> = self.notifications
            .filter(|_, n| {
                if let Some(expires_at) = n.expires_at {
                    expires_at < now
                } else {
                    false
                }
            })
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        
        let count = expired.len() as u64;
        for id in expired {
            self.notifications.remove(&id);
        }
        
        count
    }
    
    fn get_next_id(&self) -> u64 {
        let mut id = self.next_id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    }
    
    pub fn get_notification_model(&self, id: u64) -> Result<NotificationModel, ApiError> {
        self.notifications.get_or_error(&id, &format!("Notification {}", id))
    }
    
    fn count_user_notifications(&self, user: Principal) -> usize {
        self.notifications
            .filter(|_, n| n.recipient == user && !n.is_archived)
            .len()
    }
    
    fn auto_archive_old_notifications(&self, user: Principal) {
        let mut user_notifications: Vec<(u64, NotificationModel)> = self.notifications
            .filter(|_, n| n.recipient == user && !n.is_archived)
            .into_iter()
            .collect();
        
        user_notifications.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
        
        let to_archive = user_notifications.len().saturating_sub(self.max_notifications_per_user / 2);
        
        for (id, mut notification) in user_notifications.into_iter().take(to_archive) {
            notification.is_archived = true;
            self.notifications.insert(id, notification);
        }
    }
    
    fn get_category_for_type(notification_type: &NotificationType) -> NotificationCategory {
        match notification_type {
            NotificationType::TransactionReceived |
            NotificationType::TransactionSent |
            NotificationType::TransactionCompleted |
            NotificationType::TransactionCancelled |
            NotificationType::TransactionDisputed |
            NotificationType::TransactionRefunded => NotificationCategory::Transaction,
            
            NotificationType::SecurityAlert => NotificationCategory::Security,
            
            NotificationType::AccountCreated |
            NotificationType::AccountUpdated |
            NotificationType::AccountVerified |
            NotificationType::AccountFrozen => NotificationCategory::Account,
            
            NotificationType::SystemMaintenance |
            NotificationType::SystemUpdate |
            NotificationType::PolicyUpdate => NotificationCategory::System,
            
            _ => NotificationCategory::System,
        }
    }
}