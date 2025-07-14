use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::{StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

use crate::types::*;
use crate::get_memory;

pub type MessageId = u64;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub from: Principal,
    pub to: Principal,
    pub text: String,
    pub timestamp: u64,
    pub read: bool,
}

#[derive(CandidType, Deserialize)]
pub enum MessageResult {
    Ok(Message),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum PostResult {
    Ok,
    Err(String),
}

const MAX_MESSAGE_SIZE: u32 = 
    std::mem::size_of::<MessageId>() as u32 +
    std::mem::size_of::<u64>() as u32 +      
    (1 + MAX_PRINCIPAL_BYTES) * 2 +              
    (4 + MAX_TEXT_BYTES) +
    std::mem::size_of::<bool>() as u32;

impl Storable for Message {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_MESSAGE_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = Vec::with_capacity(MAX_MESSAGE_SIZE as usize);
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp.to_le_bytes());

        buf.push(self.from.as_slice().len() as u8);
        buf.extend_from_slice(self.from.as_slice());

        buf.push(self.to.as_slice().len() as u8);
        buf.extend_from_slice(self.to.as_slice());

        buf.extend_from_slice(&(self.text.as_bytes().len() as u32).to_le_bytes());
        buf.extend_from_slice(self.text.as_bytes());
        
        buf.push(if self.read { 1 } else { 0 });

        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut r = bytes.as_ref();

        let id = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];

        let timestamp = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];

        let from_len = r[0] as usize;
        r = &r[1..];
        let from = Principal::from_slice(&r[..from_len]);
        r = &r[from_len..];

        let to_len = r[0] as usize;
        r = &r[1..];
        let to = Principal::from_slice(&r[..to_len]);
        r = &r[to_len..];

        let text_len = u32::from_le_bytes(r[..4].try_into().unwrap()) as usize;
        r = &r[4..];
        let text = String::from_utf8(r[..text_len].to_vec()).unwrap();
        r = &r[text_len..];
        
        let read = r[0] == 1;

        Self { id, from, to, text, timestamp, read }
    }
}

thread_local! {
    static MESSAGES: RefCell<StableBTreeMap<MessageId, Message, crate::Memory>> =
        RefCell::new(StableBTreeMap::init(get_memory(MESSAGING_MEMORY_ID)));
    static NEXT_MESSAGE_ID: RefCell<MessageId> = RefCell::new(1);
}

#[update]
#[candid_method(update)]
pub fn post_message(to: Principal, text: String) -> PostResult {
    if text.len() > MAX_TEXT_BYTES as usize {
        return PostResult::Err(format!("Message too long ({} bytes max)", MAX_TEXT_BYTES));
    }
    
    let from = ic_cdk::caller();
    let id = NEXT_MESSAGE_ID.with(|n| {
        let mut v = n.borrow_mut();
        let current = *v;
        *v += 1;
        current
    });
    
    let msg = Message { 
        id, 
        from, 
        to, 
        text, 
        timestamp: time(),
        read: false 
    };

    MESSAGES.with(|messages| messages.borrow_mut().insert(id, msg));

    PostResult::Ok
}

#[query]
#[candid_method(query)]
pub fn get_conversation(with: Principal) -> Vec<Message> {
    let me = ic_cdk::caller();
    let mut messages: Vec<Message> = MESSAGES.with(|messages| {
        messages.borrow()
            .iter()
            .filter(|(_, m)| (m.from == me && m.to == with) || (m.from == with && m.to == me))
            .map(|(_, m)| m.clone())
            .collect()
    });

    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    messages.into_iter().take(100).collect()
}

#[update]
#[candid_method(update)]
pub fn mark_message_read(message_id: MessageId) -> PostResult {
    let caller = ic_cdk::caller();
    
    MESSAGES.with(|messages| {
        let mut messages = messages.borrow_mut();
        if let Some(mut message) = messages.get(&message_id) {
            if message.to == caller {
                message.read = true;
                messages.insert(message_id, message);
                PostResult::Ok
            } else {
                PostResult::Err("Unauthorized".to_string())
            }
        } else {
            PostResult::Err("Message not found".to_string())
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_unread_messages() -> Vec<Message> {
    let caller = ic_cdk::caller();
    MESSAGES.with(|messages| {
        messages.borrow()
            .iter()
            .filter(|(_, m)| m.to == caller && !m.read)
            .map(|(_, m)| m.clone())
            .collect()
    })
}

#[query]
#[candid_method(query)]
pub fn get_message_count() -> u64 {
    MESSAGES.with(|messages| messages.borrow().len())
}

#[query]
#[candid_method(query)]
pub fn get_messages(params: PaginationParams) -> Vec<Message> {
    let caller = ic_cdk::caller();
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    
    let mut messages: Vec<Message> = MESSAGES.with(|messages| {
        messages.borrow()
            .iter()
            .filter(|(_, m)| m.from == caller || m.to == caller)
            .map(|(_, m)| m.clone())
            .collect()
    });

    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    messages.into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect()
}