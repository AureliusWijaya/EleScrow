use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, Storable,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type MessageId = u64;

const MAX_TEXT_BYTES: u32 = 256;
const MAX_PRINCIPAL_BYTES: u32 = 29;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Message {
    id: MessageId,
    from: Principal,
    to: Principal,
    text: String,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
enum PostResult {
    Ok,
    Err(String),
}

const MAX_MESSAGE_SIZE: u32 = 
    std::mem::size_of::<MessageId>() as u32 +
    std::mem::size_of::<u64>() as u32 +      
    (1 + MAX_PRINCIPAL_BYTES) +              
    (1 + MAX_PRINCIPAL_BYTES) +              
    (4 + MAX_TEXT_BYTES);                    

impl Storable for Message {
    const BOUND: Bound = Bound::Bounded {
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

        Self { id, from, to, text, timestamp }
    }
}

thread_local! {
    static MEM: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static LOG: RefCell<StableBTreeMap<MessageId, Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEM.with(|m| m.borrow().get(MemoryId::new(0)))
        ));
    static NEXT_ID: RefCell<MessageId> = RefCell::new(0);
}

#[update]
fn post_message(to: Principal, text: String) -> PostResult {
    if text.len() > MAX_TEXT_BYTES as usize {
        return PostResult::Err(format!("msg too long ({} bytes max)", MAX_TEXT_BYTES));
    }
    let from = ic_cdk::caller();
    let id = NEXT_ID.with(|n| {
        let mut v = n.borrow_mut();
        let current = *v;
        *v += 1;
        current
    });
    let msg = Message { id, from, to, text, timestamp: time() };

    LOG.with(|log| log.borrow_mut().insert(id, msg));

    PostResult::Ok
}

#[query]
fn get_conversation(with: Principal) -> Vec<Message> {
    let me = ic_cdk::caller();
    let mut messages: Vec<Message> = LOG.with(|log| {
        log.borrow()
            .iter()
            .filter(|(_, m)| (m.from == me && m.to == with) || (m.from == with && m.to == me))
            .map(|(_, m)| m.clone())
            .collect()
    });

    messages.reverse();
    messages.into_iter().take(100).collect()
}
