use crate::types::{ConversationKey, Message, PaginationParams, PostResult, WsEvent};
use crate::{get_memory, types::MAX_TEXT_BYTES};
use candid::{candid_method, Principal};
use ic_cdk::api::{self, time};
use ic_cdk_macros::*;
use ic_stable_structures::{StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell, collections::HashSet};
use ic_websocket_cdk::{send};

const MESSAGES_MEMORY_ID: u8 = 0;
const CONVERSATION_INDEX_MEMORY_ID: u8 = 1;


thread_local! {
    static MESSAGES: RefCell<StableBTreeMap<u64, Message, crate::Memory>> =
        RefCell::new(StableBTreeMap::init(get_memory(MESSAGES_MEMORY_ID)));

    static CONVERSATION_INDEX: RefCell<StableBTreeMap<Vec<u8>, (), crate::Memory>> =
        RefCell::new(StableBTreeMap::init(get_memory(CONVERSATION_INDEX_MEMORY_ID)));

    static NEXT_MESSAGE_ID: RefCell<u64> = RefCell::new(1);

    static ONLINE_USERS: RefCell<HashSet<Principal>> = RefCell::new(HashSet::new());
}

pub fn on_client_open(principal: Principal) {
    ONLINE_USERS.with_borrow_mut(|online_users| {
        online_users.insert(principal);
    });
}

pub fn on_client_close(principal: Principal) {
    ONLINE_USERS.with_borrow_mut(|online_users| {
        online_users.remove(&principal);
    });
}

pub fn on_ws_message(principal: Principal, message: Vec<u8>) {
    if message == b"ping" {
        let _ = send(principal, b"pong".to_vec());
    }
}

fn send_ws_event(user: Principal, event: WsEvent) {
    match candid::encode_one(&event) {
        Ok(event_bytes) => {
            ONLINE_USERS.with_borrow(|online_users| {
                if online_users.contains(&user) {
                    match send(user, event_bytes) {
                        Ok(_) => println!("WebSocket message sent to user: {}", user),
                        Err(e) => eprintln!("Failed to send WebSocket message: {:?}", e),
                    }
                }
            });
        }
        Err(e) => {
            eprintln!("Failed to encode WsEvent: {:?}", e);
        }
    }
}

#[update]
#[candid_method(update)]
pub fn post_message(to: Principal, text: String) -> PostResult {
    if text.len() > MAX_TEXT_BYTES as usize {
        return PostResult::Err(format!("Message exceeds {} bytes limit", MAX_TEXT_BYTES));
    }

    let from = api::msg_caller();
    let id = NEXT_MESSAGE_ID.with_borrow_mut(|id_ref| {
        let id = *id_ref;
        *id_ref += 1;
        id
    });

    let message = Message {
        id,
        from,
        to,
        text,
        timestamp: time(),
        read: false,
    };

    MESSAGES.with_borrow_mut(|m| m.insert(id, message.clone()));

    let conv_key = ConversationKey::new(from, to);
    let index_key = build_index_key(&conv_key, message.timestamp, id);
    CONVERSATION_INDEX.with_borrow_mut(|index| index.insert(index_key, ()));

    send_ws_event(to, WsEvent::NewMessage(message.clone()));
    send_ws_event(from, WsEvent::NewMessage(message));

    PostResult::Ok
}

#[query]
#[candid_method(query)]
pub fn get_conversation_chunk(with: Principal, params: PaginationParams) -> Vec<Message> {
    let me = api::msg_caller();
    let conv_key = ConversationKey::new(me, with);
    let prefix = conv_key.to_bytes();

    let limit = params.limit.unwrap_or(50).min(100) as usize;
    let offset = params.offset.unwrap_or(0) as usize;

    CONVERSATION_INDEX.with_borrow(|index| {
        index
            .iter()
            .filter(|(key, _)| key.starts_with(&prefix))
            .skip(offset)
            .take(limit)
            .filter_map(|(key, _)| {
                let (_, _, message_id) = parse_index_key(&key);
                MESSAGES.with_borrow(|m| m.get(&message_id))
            })
            .collect()
    })
}

#[update]
#[candid_method(update)]
pub fn mark_message_read(message_id: u64) -> PostResult {
    let caller = api::msg_caller();
    MESSAGES.with_borrow_mut(|messages| {
        if let Some(mut message) = messages.get(&message_id) {
            if message.to != caller {
                return PostResult::Err("Not message recipient".to_string());
            }
            if !message.read {
                message.read = true;
                messages.insert(message_id, message.clone());
                send_ws_event(message.from, WsEvent::MessageRead { message_id });
                send_ws_event(caller, WsEvent::MessageRead { message_id });
            }
            PostResult::Ok
        } else {
            PostResult::Err("Message not found".to_string())
        }
    })
}

fn build_index_key(conv_key: &ConversationKey, timestamp: u64, message_id: u64) -> Vec<u8> {
    let mut key = conv_key.to_bytes().into_owned();
    key.extend((u64::MAX - timestamp).to_be_bytes());
    key.extend(message_id.to_be_bytes());
    key
}

fn parse_index_key(key: &[u8]) -> (ConversationKey, u64, u64) {
    let conv_key = ConversationKey::from_bytes(Cow::Borrowed(key));
    let timestamp_offset = conv_key.to_bytes().len();
    let rev_timestamp =
        u64::from_be_bytes(key[timestamp_offset..timestamp_offset + 8].try_into().unwrap());
    let message_id = u64::from_be_bytes(key[timestamp_offset + 8..].try_into().unwrap());
    (conv_key, u64::MAX - rev_timestamp, message_id)
}

#[query]
#[candid_method(query)]
pub fn get_message_count() -> u64 {
    MESSAGES.with(|messages| messages.borrow().len())
}