use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Notification {
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


#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    TransactionReceived,
    TransactionSent,
    TransactionCompleted,
    TransactionCancelled,
    TransactionDisputed,
    TransactionRefunded,
    
    EscrowCreated,
    EscrowReleased,
    EscrowDisputed,
    EscrowExpiring,
    
    AccountCreated,
    AccountUpdated,
    AccountVerified,
    AccountFrozen,
    SecurityAlert,
    
    SystemMaintenance,
    SystemUpdate,
    PolicyUpdate,
    
    NewMessage,
    MessageReply,
    
    Custom { type_name: String },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
    Critical,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum NotificationCategory {
    Transaction,
    Security,
    Account,
    System,
    Marketing,
    Social,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum RelatedResource {
    Transaction(u64),
    User(Principal),
    Message(u64),
    Dispute(u64),
    Document(String),
    ExternalUrl(String),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationAction {
    pub id: String,
    pub label: String,
    pub action_type: ActionType,
    pub style: ActionStyle,
    pub confirmation_required: bool,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ActionType {
    Navigate { url: String },
    Approve { resource_id: String },
    Reject { resource_id: String },
    Dismiss,
    Custom { action: String, data: String },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ActionStyle {
    Primary,
    Secondary,
    Danger,
    Success,
    Link,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct DeliveryStatus {
    pub in_app: DeliveryState,
    pub email: Option<DeliveryState>,
    pub sms: Option<DeliveryState>,
    pub push: Option<DeliveryState>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum DeliveryState {
    Pending,
    Sent { sent_at: u64 },
    Delivered { delivered_at: u64 },
    Failed { failed_at: u64, reason: String },
    Bounced { bounced_at: u64, reason: String },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationFilter {
    pub unread_only: Option<bool>,
    pub priority: Option<Vec<NotificationPriority>>,
    pub notification_type: Option<Vec<NotificationType>>,
    pub category: Option<Vec<NotificationCategory>>,
    pub date_range: Option<crate::types::common::TimeFilter>,
    pub has_actions: Option<bool>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<Notification>,
    pub total: u64,
    pub unread_count: u64,
    pub unread_by_priority: Vec<(NotificationPriority, u64)>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UpdateNotificationPreferences {
    pub email_enabled: Option<bool>,
    pub sms_enabled: Option<bool>,
    pub push_enabled: Option<bool>,
    pub transaction_alerts: Option<bool>,
    pub security_alerts: Option<bool>,
    pub marketing_emails: Option<bool>,
    pub quiet_hours: Option<QuietHours>,
    pub frequency: Option<NotificationFrequency>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct QuietHours {
    pub enabled: bool,
    pub start_hour: u8,
    pub end_hour: u8,
    pub timezone: String,
    pub exclude_urgent: bool,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Instant,
    Hourly,
    Daily,
    Weekly,
    Never,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub notification_type: NotificationType,
    pub title_template: String,
    pub message_template: String,
    pub default_priority: NotificationPriority,
    pub variables: Vec<String>,
}

impl Default for DeliveryStatus {
    fn default() -> Self {
        Self {
            in_app: DeliveryState::Pending,
            email: None,
            sms: None,
            push: None,
        }
    }
}

impl Default for NotificationPriority {
    fn default() -> Self {
        NotificationPriority::Normal
    }
}