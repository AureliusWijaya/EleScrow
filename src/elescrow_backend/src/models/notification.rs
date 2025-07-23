use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::Storable;
use std::borrow::Cow;
use crate::types::notification::*;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationModel {
    pub id: u64,
    pub recipient: Principal,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub category: NotificationCategory,
    pub related_resource: Option<RelatedResource>,
    pub actions: Vec<NotificationAction>,
    pub is_read: bool,
    pub is_archived: bool,
    pub created_at: u64,
    pub read_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub delivery_status: DeliveryStatus,
}

impl Storable for NotificationModel {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = serde_cbor::to_vec(self).expect("Failed to serialize NotificationModel");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).expect("Failed to deserialize NotificationModel")
    }
}

impl From<NotificationModel> for Notification {
    fn from(model: NotificationModel) -> Self {
        Notification {
            id: model.id,
            recipient: model.recipient,
            notification_type: model.notification_type,
            title: model.title,
            message: model.message,
            priority: model.priority,
            category: model.category,
            related_resource: model.related_resource,
            actions: model.actions,
            is_read: model.is_read,
            is_archived: model.is_archived,
            created_at: model.created_at,
            read_at: model.read_at,
            expires_at: model.expires_at,
            delivery_status: model.delivery_status,
        }
    }
}