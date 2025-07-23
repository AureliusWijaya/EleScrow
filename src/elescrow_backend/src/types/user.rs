use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserProfile {
    pub principal: Principal,
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_active: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserAccount {
    pub is_active: bool,
    pub is_verified: bool,
    pub is_frozen: bool,
    pub freeze_reason: Option<String>,
    pub verification_level: VerificationLevel,
    pub kyc_status: KycStatus,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum VerificationLevel {
    Basic,
    Standard,
    Enhanced,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum KycStatus {
    NotStarted,
    Pending,
    UnderReview,
    Approved,
    Rejected { reason: String },
    Expired,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub transaction_alerts: bool,
    pub security_alerts: bool,
    pub marketing_emails: bool,
    pub weekly_summary: bool,
    pub notification_channels: Vec<NotificationChannel>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    SMS,
    InApp,
    Push,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub two_factor_enabled: bool,
    pub two_factor_method: Option<TwoFactorMethod>,
    pub allowed_ips: Vec<String>,
    pub session_timeout: u64,
    pub require_password_change: bool,
    pub last_password_change: u64,
    pub login_history: Vec<LoginAttempt>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum TwoFactorMethod {
    SMS,
    Email,
    AuthenticatorApp,
    HardwareKey,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub timestamp: u64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub success: bool,
    pub failure_reason: Option<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct User {
    pub profile: UserProfile,
    pub account: UserAccount,
    pub notification_preferences: NotificationPreferences,
    pub security_settings: SecuritySettings,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub referral_code: Option<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserSearchParams {
    pub query: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    pub is_active: Option<bool>,
    pub created_after: Option<u64>,
    pub created_before: Option<u64>,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            email_enabled: true,
            sms_enabled: false,
            push_enabled: true,
            transaction_alerts: true,
            security_alerts: true,
            marketing_emails: false,
            weekly_summary: true,
            notification_channels: vec![NotificationChannel::Email, NotificationChannel::InApp],
        }
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            two_factor_enabled: false,
            two_factor_method: None,
            allowed_ips: vec![],
            session_timeout: 86400 * 1_000_000_000,
            require_password_change: false,
            last_password_change: 0,
            login_history: vec![],
        }
    }
}