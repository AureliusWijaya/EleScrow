use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::Storable;
use std::borrow::Cow;
use crate::types::user::*;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserModel {
    pub principal: Principal,
    pub profile: UserProfile,
    pub account: UserAccount,
    pub notification_preferences: NotificationPreferences,
    pub security_settings: SecuritySettings,
}

impl Storable for UserModel {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = serde_cbor::to_vec(self).expect("Failed to serialize UserModel");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).expect("Failed to deserialize UserModel")
    }
}

impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            profile: model.profile,
            account: model.account,
            notification_preferences: model.notification_preferences,
            security_settings: model.security_settings,
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            principal: user.profile.principal,
            profile: user.profile,
            account: user.account,
            notification_preferences: user.notification_preferences,
            security_settings: user.security_settings,
        }
    }
}