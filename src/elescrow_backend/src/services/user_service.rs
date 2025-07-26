use candid::Principal;
use ic_cdk::api::time;
use std::cell::RefCell;

use crate::types::{
    errors::ApiError,
    user::*,
};

use crate::models::user::UserModel;
use crate::storage::{
    stable_storage::StableStorage,
    memory::MemoryRegion,
};
use crate::security::{
    validation,
    audit::AuditLogger,
};

use crate::types::common::AuditAction;

pub struct UserService {
    users: StableStorage<Principal, UserModel>,
    usernames: StableStorage<String, Principal>,
    emails: StableStorage<String, Principal>,
    
    audit_logger: RefCell<AuditLogger>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: StableStorage::new(MemoryRegion::Users),
            usernames: StableStorage::new(MemoryRegion::UserIndex),
            emails: StableStorage::new(MemoryRegion::UserIndex),
            audit_logger: RefCell::new(AuditLogger::with_defaults()),
        }
    }
    
    pub fn register(
        &self,
        principal: Principal,
        request: RegisterUserRequest,
    ) -> Result<User, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        // validation::validate_principal(&principal)?;
        validation::validate_username(&request.username)?;
        
        if let Some(email) = &request.email {
            validation::validate_email(email)?;
        }
        
        if self.users.contains_key(&principal) {
            return Err(ApiError::AlreadyExists {
                resource: "User".to_string(),
            });
        }
        
        if self.usernames.contains_key(&request.username) {
            return Err(ApiError::AlreadyExists {
                resource: "Username".to_string(),
            });
        }
        
        if let Some(email) = &request.email {
            if self.emails.contains_key(email) {
                return Err(ApiError::AlreadyExists {
                    resource: "Email".to_string(),
                });
            }
        }
        
        let now = time();
        let user_model = UserModel {
            principal,
            profile: UserProfile {
                principal,
                username: request.username.clone(),
                display_name: request.display_name.unwrap_or(request.username.clone()),
                bio: String::new(),
                avatar_url: None,
                email: request.email.clone(),
                phone: None,
                created_at: now,
                updated_at: now,
                last_active: now,
            },
            account: UserAccount {
                is_active: true,
                is_verified: false,
                is_frozen: false,
                freeze_reason: None,
                verification_level: VerificationLevel::Basic,
                kyc_status: KycStatus::NotStarted,
            },
            notification_preferences: NotificationPreferences::default(),
            security_settings: SecuritySettings::default(),
        };
        
        self.users.insert(principal, user_model.clone());
        self.usernames.insert(request.username.clone(), principal);
        
        if let Some(email) = &request.email {
            self.emails.insert(email.clone(), principal);
        }
        
        self.audit_logger.borrow().log(
            principal,
            AuditAction::UserRegistered,
            &principal.to_text(),
            Some(format!("Username: {}", request.username)),
        );
        
        Ok(user_model.into())
    }
    
    pub fn get_user(&self, principal: Principal) -> Result<User, ApiError> {
        let user_model = self.users.get_or_error(&principal, "User")?;
        
        self.update_last_active(principal);
        
        Ok(user_model.into())
    }
    
    pub fn get_user_by_username(&self, username: &str) -> Result<User, ApiError> {
        let principal = self.usernames.get_or_error(&username.to_string(), "Username")?;
        self.get_user(principal)
    }
    
    pub fn update_profile(
        &self,
        principal: Principal,
        request: UpdateProfileRequest,
    ) -> Result<User, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        if let Some(display_name) = request.display_name {
            let sanitized = validation::validate_text(&display_name, "display_name", 1, 100)?;
            user_model.profile.display_name = sanitized;
        }
        
        if let Some(bio) = request.bio {
            let sanitized = validation::validate_text(&bio, "bio", 0, 500)?;
            user_model.profile.bio = sanitized;
        }
        
        if let Some(avatar_url) = request.avatar_url {
            validation::validate_url(&avatar_url)?;
            user_model.profile.avatar_url = Some(avatar_url);
        }
        
        if let Some(email) = request.email {
            validation::validate_email(&email)?;
            
            if let Some(old_email) = &user_model.profile.email {
                self.emails.remove(old_email);
            }
            self.emails.insert(email.clone(), principal);
            user_model.profile.email = Some(email);
        }
        
        if let Some(phone) = request.phone {
            validation::validate_phone(&phone)?;
            user_model.profile.phone = Some(phone);
        }
        
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model.clone());
        
        self.audit_logger.borrow().log(
            principal,
            AuditAction::UserUpdated,
            &principal.to_text(),
            Some("Profile updated".to_string()),
        );
        
        Ok(user_model.into())
    }
    
    pub fn update_notification_preferences(
        &self,
        principal: Principal,
        preferences: NotificationPreferences,
    ) -> Result<User, ApiError> {
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        user_model.notification_preferences = preferences;
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model.clone());
        
        Ok(user_model.into())
    }
    
    pub fn update_security_settings(
        &self,
        principal: Principal,
        settings: SecuritySettings,
    ) -> Result<User, ApiError> {
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        user_model.security_settings = settings;
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model.clone());
        
        self.audit_logger.borrow().log(
            principal,
            AuditAction::UserUpdated,
            &principal.to_text(),
            Some("Security settings updated".to_string()),
        );
        
        Ok(user_model.into())
    }
    
    pub fn deactivate_account(&self, principal: Principal) -> Result<(), ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        user_model.account.is_active = false;
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model);
        
        self.audit_logger.borrow().log(
            principal,
            AuditAction::UserDeactivated,
            &principal.to_text(),
            None,
        );
        
        Ok(())
    }
    
    pub fn reactivate_account(&self, principal: Principal) -> Result<(), ApiError> {
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        if user_model.account.is_frozen {
            return Err(ApiError::AccountFrozen {
                reason: user_model.account.freeze_reason.unwrap_or_default(),
            });
        }
        
        user_model.account.is_active = true;
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model);
        
        self.audit_logger.borrow().log(
            principal,
            AuditAction::UserReactivated,
            &principal.to_text(),
            None,
        );
        
        Ok(())
    }

    pub fn search_users(
        &self,
        params: UserSearchParams,
        pagination: crate::types::common::PaginationParams,
    ) -> Result<Vec<User>, ApiError> {
        pagination.validate()?;

        let users: Vec<User> = self.users
            .entries()
            .into_iter()
            .filter_map(|(_, user_model)| {
                let user: User = user_model.into();

                if let Some(query) = &params.query {
                    let query_lower = query.to_lowercase();
                    if !user.profile.username.to_lowercase().contains(&query_lower) &&
                       !user.profile.display_name.to_lowercase().contains(&query_lower) {
                        return None;
                    }
                }

                if let Some(level) = &params.verification_level {
                    if &user.account.verification_level != level {
                        return None;
                    }
                }

                if let Some(is_active) = params.is_active {
                    if user.account.is_active != is_active {
                        return None;
                    }
                }

                if let Some(created_after) = params.created_after {
                    if user.profile.created_at < created_after {
                        return None;
                    }
                }

                if let Some(created_before) = params.created_before {
                    if user.profile.created_at > created_before {
                        return None;
                    }
                }

                Some(user)
            })
            .skip(pagination.offset as usize)
            .take(pagination.limit as usize)
            .collect();

        Ok(users)
    }
    
    pub fn freeze_account(
        &self,
        principal: Principal,
        reason: String,
        frozen_by: Principal,
    ) -> Result<(), ApiError> {
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        user_model.account.is_frozen = true;
        user_model.account.freeze_reason = Some(reason.clone());
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model);
        
        self.audit_logger.borrow().log(
            frozen_by,
            AuditAction::AccountFrozen,
            &principal.to_text(),
            Some(format!("Reason: {}", reason)),
        );
        
        Ok(())
    }
    
    pub fn unfreeze_account(
        &self,
        principal: Principal,
        unfrozen_by: Principal,
    ) -> Result<(), ApiError> {
        let mut user_model = self.users.get_or_error(&principal, "User")?;
        
        user_model.account.is_frozen = false;
        user_model.account.freeze_reason = None;
        user_model.profile.updated_at = time();
        
        self.users.insert(principal, user_model);
        
        self.audit_logger.borrow().log(
            unfrozen_by,
            AuditAction::AccountUnfrozen,
            &principal.to_text(),
            None,
        );
        
        Ok(())
    }
    
    pub fn admin_update_verification_status(
        &self,
        user_principal: Principal,
        verification_level: VerificationLevel,
        admin_principal: Principal,
    ) -> Result<User, ApiError> {
        let mut user = self.get_user(user_principal)?;

        user.account.is_verified = true;
        user.account.verification_level = verification_level.clone();
        user.profile.updated_at = time();

        self.users.insert(user_principal, user.clone().into());

        self.audit_logger.borrow().log(
            admin_principal,
            AuditAction::KycStatusUpdated,
            &user_principal.to_string(),
            Some(format!("User verified to level: {:?}", verification_level)),
        );

        Ok(user)
    }

    pub fn get_user_statistics(&self) -> UserStatistics {
        let total_users = self.users.len();
        let now = time();
        let day_ago = now - (24 * 60 * 60 * 1_000_000_000);
        
        let mut active_users = 0;
        let mut verified_users = 0;
        let mut frozen_accounts = 0;
        
        for (_, user) in self.users.entries() {
            if user.profile.last_active >= day_ago {
                active_users += 1;
            }
            if user.account.is_verified {
                verified_users += 1;
            }
            if user.account.is_frozen {
                frozen_accounts += 1;
            }
        }
        
        UserStatistics {
            total_users,
            active_users_24h: active_users,
            verified_users,
            frozen_accounts,
            new_users_today: 0,
        }
    }
    
    fn update_last_active(&self, principal: Principal) {
        if let Some(mut user) = self.users.get(&principal) {
            user.profile.last_active = time();
            self.users.insert(principal, user);
        }
    }
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct UserStatistics {
    pub total_users: u64,
    pub active_users_24h: u64,
    pub verified_users: u64,
    pub frozen_accounts: u64,
    pub new_users_today: u64,
}