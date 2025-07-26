use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct CanisterInfo {
    pub name: String,
    pub version: String,
    pub modules: Vec<String>,
    pub total_memory_usage: u64,
}

#[derive(CandidType, Deserialize)]
pub enum PostResult {
    Ok,
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum ApiResult<T> {
    Ok(T),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum BoolResult {
    Ok(bool),
    Err(String),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct User {
    pub id: Principal,
    pub username: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct PaginationParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(50),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConversationKey {
    pub user1: Principal,
    pub user2: Principal,
}

impl ConversationKey {
    pub fn new(user1: Principal, user2: Principal) -> Self {
        if user1 < user2 {
            Self { user1, user2 }
        } else {
            Self {
                user1: user2,
                user2: user1,
            }
        }
    }
}

impl Storable for ConversationKey {
    const BOUND: Bound = Bound::Bounded {
        max_size: (1 + MAX_PRINCIPAL_BYTES) * 2,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = Vec::new();
        let user1_slice = self.user1.as_slice();
        bytes.push(user1_slice.len() as u8);
        bytes.extend(user1_slice);
        let user2_slice = self.user2.as_slice();
        bytes.push(user2_slice.len() as u8);
        bytes.extend(user2_slice);
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let bytes = bytes.as_ref();
        let user1_len = bytes[0] as usize;
        let user1 = Principal::from_slice(&bytes[1..1 + user1_len]);
        let user2_len = bytes[1 + user1_len] as usize;
        let user2 = Principal::from_slice(&bytes[2 + user1_len..2 + user1_len + user2_len]);
        Self { user1, user2 }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub from: Principal,
    pub to: Principal,
    pub text: String,
    pub timestamp: u64,
    pub read: bool,
}

impl Storable for Message {
    const BOUND: Bound = Bound::Bounded {
        max_size: 8 + 8 + (1 + MAX_PRINCIPAL_BYTES) * 2 + 4 + MAX_TEXT_BYTES + 1,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = Vec::with_capacity(Self::BOUND.max_size() as usize);
        buf.extend(self.id.to_le_bytes());
        buf.extend(self.timestamp.to_le_bytes());

        let from_slice = self.from.as_slice();
        buf.push(from_slice.len() as u8);
        buf.extend(from_slice);

        let to_slice = self.to.as_slice();
        buf.push(to_slice.len() as u8);
        buf.extend(to_slice);

        let text_bytes = self.text.as_bytes();
        buf.extend((text_bytes.len() as u32).to_le_bytes());
        buf.extend(text_bytes);

        buf.push(self.read as u8);

        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut offset = 0;

        let id = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let timestamp = u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let from_len = bytes[offset] as usize;
        offset += 1;
        let from = Principal::from_slice(&bytes[offset..offset + from_len]);
        offset += from_len;

        let to_len = bytes[offset] as usize;
        offset += 1;
        let to = Principal::from_slice(&bytes[offset..offset + to_len]);
        offset += to_len;

        let text_len = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let text = String::from_utf8_lossy(&bytes[offset..offset + text_len]).into_owned();
        offset += text_len;

        let read = bytes[offset] != 0;

        Self {
            id,
            from,
            to,
            text,
            timestamp,
            read,
        }
    }
}

#[derive(CandidType, Clone, Debug)]
pub enum WsEvent {
    NewMessage(Message),
    MessageRead { message_id: u64 },
}

pub const TRANSACTIONS_MEMORY_ID: u8 = 1;
pub const ESCROWS_MEMORY_ID: u8 = 2;
pub const BALANCES_MEMORY_ID: u8 = 3;
pub const USERS_MEMORY_ID: u8 = 4;

pub const MAX_TEXT_BYTES: u32 = 1000;
pub const MAX_PRINCIPAL_BYTES: u32 = 29;
pub const MAX_METADATA_BYTES: u32 = 2000;