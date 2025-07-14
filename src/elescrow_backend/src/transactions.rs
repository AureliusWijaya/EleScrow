use candid::{candid_method, CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::{StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};

use crate::types::*;
use crate::get_memory;

pub type TransactionId = u64;
pub type Balance = u64;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Completed,
    Cancelled,
    Disputed,
    Refunded,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Escrow,
    DirectPayment,
    Refund,
    Dispute,
    Release,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub transaction_type: TransactionType,
    pub from: Principal,
    pub to: Principal,
    pub amount: Balance,
    pub description: String,
    pub status: TransactionStatus,
    pub escrow_agent: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
    pub deadline: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UserBalance {
    pub user: Principal,
    pub available: Balance,
    pub locked: Balance,
    pub updated_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct CreateTransactionRequest {
    pub transaction_type: TransactionType,
    pub to: Principal,
    pub amount: Balance,
    pub description: String,
    pub escrow_agent: Option<Principal>,
    pub deadline: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub enum TransactionResult {
    Ok(Transaction),
    Err(String),
}

#[derive(CandidType, Deserialize)]
pub enum BalanceResult {
    Ok(UserBalance),
    Err(String),
}

const MAX_TRANSACTION_SIZE: u32 = 
    std::mem::size_of::<TransactionId>() as u32 +
    std::mem::size_of::<u8>() as u32 * 2 +
    (1 + MAX_PRINCIPAL_BYTES) * 3 +
    std::mem::size_of::<Balance>() as u32 +
    (4 + MAX_TEXT_BYTES) +
    std::mem::size_of::<u64>() as u32 * 3;

const MAX_BALANCE_SIZE: u32 = 
    (1 + MAX_PRINCIPAL_BYTES) +
    std::mem::size_of::<Balance>() as u32 * 2 +
    std::mem::size_of::<u64>() as u32;

impl Storable for Transaction {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_TRANSACTION_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = Vec::with_capacity(MAX_TRANSACTION_SIZE as usize);
        
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.push(transaction_type_to_u8(&self.transaction_type));
        buf.push(transaction_status_to_u8(&self.status));
        
        buf.push(self.from.as_slice().len() as u8);
        buf.extend_from_slice(self.from.as_slice());
        
        buf.push(self.to.as_slice().len() as u8);
        buf.extend_from_slice(self.to.as_slice());
        
        if let Some(ref agent) = self.escrow_agent {
            buf.push(1);
            buf.push(agent.as_slice().len() as u8);
            buf.extend_from_slice(agent.as_slice());
        } else {
            buf.push(0);
        }
        
        buf.extend_from_slice(&self.amount.to_le_bytes());
        
        buf.extend_from_slice(&(self.description.as_bytes().len() as u32).to_le_bytes());
        buf.extend_from_slice(self.description.as_bytes());
        
        buf.extend_from_slice(&self.created_at.to_le_bytes());
        buf.extend_from_slice(&self.updated_at.to_le_bytes());
        
        if let Some(deadline) = self.deadline {
            buf.push(1);
            buf.extend_from_slice(&deadline.to_le_bytes());
        } else {
            buf.push(0);
        }
        
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut r = bytes.as_ref();
        
        let id = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let transaction_type = u8_to_transaction_type(r[0]);
        r = &r[1..];
        
        let status = u8_to_transaction_status(r[0]);
        r = &r[1..];
        
        let from_len = r[0] as usize;
        r = &r[1..];
        let from = Principal::from_slice(&r[..from_len]);
        r = &r[from_len..];
        
        let to_len = r[0] as usize;
        r = &r[1..];
        let to = Principal::from_slice(&r[..to_len]);
        r = &r[to_len..];
        
        let escrow_agent = if r[0] == 1 {
            r = &r[1..];
            let agent_len = r[0] as usize;
            r = &r[1..];
            let agent = Principal::from_slice(&r[..agent_len]);
            r = &r[agent_len..];
            Some(agent)
        } else {
            r = &r[1..];
            None
        };
        
        let amount = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let desc_len = u32::from_le_bytes(r[..4].try_into().unwrap()) as usize;
        r = &r[4..];
        let description = String::from_utf8(r[..desc_len].to_vec()).unwrap();
        r = &r[desc_len..];
        
        let created_at = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let updated_at = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let deadline = if r[0] == 1 {
            r = &r[1..];
            let deadline = u64::from_le_bytes(r[..8].try_into().unwrap());
            Some(deadline)
        } else {
            None
        };
        
        Self {
            id,
            transaction_type,
            from,
            to,
            amount,
            description,
            status,
            escrow_agent,
            created_at,
            updated_at,
            deadline,
        }
    }
}

impl Storable for UserBalance {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_BALANCE_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = Vec::with_capacity(MAX_BALANCE_SIZE as usize);
        
        buf.push(self.user.as_slice().len() as u8);
        buf.extend_from_slice(self.user.as_slice());
        
        buf.extend_from_slice(&self.available.to_le_bytes());
        buf.extend_from_slice(&self.locked.to_le_bytes());
        buf.extend_from_slice(&self.updated_at.to_le_bytes());
        
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut r = bytes.as_ref();
        
        let user_len = r[0] as usize;
        r = &r[1..];
        let user = Principal::from_slice(&r[..user_len]);
        r = &r[user_len..];
        
        let available = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let locked = u64::from_le_bytes(r[..8].try_into().unwrap());
        r = &r[8..];
        
        let updated_at = u64::from_le_bytes(r[..8].try_into().unwrap());
        
        Self {
            user,
            available,
            locked,
            updated_at,
        }
    }
}

// Helper functions for enum serialization
fn transaction_type_to_u8(t: &TransactionType) -> u8 {
    match t {
        TransactionType::Escrow => 0,
        TransactionType::DirectPayment => 1,
        TransactionType::Refund => 2,
        TransactionType::Dispute => 3,
        TransactionType::Release => 4,
    }
}

fn u8_to_transaction_type(n: u8) -> TransactionType {
    match n {
        0 => TransactionType::Escrow,
        1 => TransactionType::DirectPayment,
        2 => TransactionType::Refund,
        3 => TransactionType::Dispute,
        4 => TransactionType::Release,
        _ => TransactionType::DirectPayment,
    }
}

fn transaction_status_to_u8(s: &TransactionStatus) -> u8 {
    match s {
        TransactionStatus::Pending => 0,
        TransactionStatus::Approved => 1,
        TransactionStatus::Completed => 2,
        TransactionStatus::Cancelled => 3,
        TransactionStatus::Disputed => 4,
        TransactionStatus::Refunded => 5,
    }
}

fn u8_to_transaction_status(n: u8) -> TransactionStatus {
    match n {
        0 => TransactionStatus::Pending,
        1 => TransactionStatus::Approved,
        2 => TransactionStatus::Completed,
        3 => TransactionStatus::Cancelled,
        4 => TransactionStatus::Disputed,
        5 => TransactionStatus::Refunded,
        _ => TransactionStatus::Pending,
    }
}

thread_local! {
    static TRANSACTIONS: RefCell<StableBTreeMap<TransactionId, Transaction, crate::Memory>> =
        RefCell::new(StableBTreeMap::init(get_memory(TRANSACTIONS_MEMORY_ID)));
    
    static BALANCES: RefCell<StableBTreeMap<Principal, UserBalance, crate::Memory>> =
        RefCell::new(StableBTreeMap::init(get_memory(BALANCES_MEMORY_ID)));
    
    static NEXT_TX_ID: RefCell<TransactionId> = RefCell::new(1);
}

#[update]
#[candid_method(update)]
pub fn create_transaction(request: CreateTransactionRequest) -> TransactionResult {
    let caller = ic_cdk::caller();
    
    if request.amount == 0 {
        return TransactionResult::Err("Amount must be greater than 0".to_string());
    }
    
    let user_balance = get_user_balance_internal(caller);
    if user_balance.available < request.amount {
        return TransactionResult::Err("Insufficient balance".to_string());
    }
    
    let tx_id = NEXT_TX_ID.with(|id| {
        let mut current = id.borrow_mut();
        let new_id = *current;
        *current += 1;
        new_id
    });
    
    let now = time();
    let transaction = Transaction {
        id: tx_id,
        transaction_type: request.transaction_type,
        from: caller,
        to: request.to,
        amount: request.amount,
        description: request.description,
        status: TransactionStatus::Pending,
        escrow_agent: request.escrow_agent,
        created_at: now,
        updated_at: now,
        deadline: request.deadline,
    };
    
    if transaction.transaction_type == TransactionType::Escrow {
        lock_funds(caller, request.amount);
    }
    
    TRANSACTIONS.with(|tx| tx.borrow_mut().insert(tx_id, transaction.clone()));
    
    TransactionResult::Ok(transaction)
}

#[update]
#[candid_method(update)]
pub fn deposit(amount: Balance) -> BalanceResult {
    let caller = ic_cdk::caller();
    
    if amount == 0 {
        return BalanceResult::Err("Amount must be greater than 0".to_string());
    }
    
    let mut balance = get_user_balance_internal(caller);
    balance.available += amount;
    balance.updated_at = time();
    
    BALANCES.with(|balances| balances.borrow_mut().insert(caller, balance.clone()));
    
    BalanceResult::Ok(balance)
}

#[update]
#[candid_method(update)]
pub fn withdraw(amount: Balance) -> BalanceResult {
    let caller = ic_cdk::caller();
    
    if amount == 0 {
        return BalanceResult::Err("Amount must be greater than 0".to_string());
    }
    
    let mut balance = get_user_balance_internal(caller);
    
    if balance.available < amount {
        return BalanceResult::Err("Insufficient balance".to_string());
    }
    
    balance.available -= amount;
    balance.updated_at = time();
    
    BALANCES.with(|balances| balances.borrow_mut().insert(caller, balance.clone()));
    
    BalanceResult::Ok(balance)
}

#[query]
#[candid_method(query)]
pub fn get_transaction(transaction_id: TransactionId) -> TransactionResult {
    match TRANSACTIONS.with(|tx| tx.borrow().get(&transaction_id)) {
        Some(transaction) => TransactionResult::Ok(transaction),
        None => TransactionResult::Err("Transaction not found".to_string()),
    }
}

#[query]
#[candid_method(query)]
pub fn get_user_transactions(params: PaginationParams) -> Vec<Transaction> {
    let caller = ic_cdk::caller();
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    
    let mut transactions: Vec<Transaction> = TRANSACTIONS.with(|tx| {
        tx.borrow()
            .iter()
            .filter(|(_, t)| t.from == caller || t.to == caller)
            .map(|(_, t)| t.clone())
            .collect()
    });
    
    transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    transactions.into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect()
}

#[query]
#[candid_method(query)]
pub fn get_user_balance() -> BalanceResult {
    let caller = ic_cdk::caller();
    let balance = get_user_balance_internal(caller);
    BalanceResult::Ok(balance)
}

#[query]
#[candid_method(query)]
pub fn get_transaction_count() -> u64 {
    TRANSACTIONS.with(|tx| tx.borrow().len())
}

#[update]
#[candid_method(update)]
pub fn cancel_transaction(transaction_id: TransactionId) -> BoolResult {
    let caller = ic_cdk::caller();
    
    let transaction = match TRANSACTIONS.with(|tx| tx.borrow().get(&transaction_id)) {
        Some(tx) => tx,
        None => return BoolResult::Err("Transaction not found".to_string()),
    };
    
    if transaction.from != caller {
        return BoolResult::Err("Unauthorized to cancel this transaction".to_string());
    }
    
    if transaction.status != TransactionStatus::Pending {
        return BoolResult::Err("Can only cancel pending transactions".to_string());
    }
    
    if transaction.transaction_type == TransactionType::Escrow {
        unlock_funds(transaction.from, transaction.amount);
    }
    
    update_transaction_status(transaction_id, TransactionStatus::Cancelled);
    
    BoolResult::Ok(true)
}

// Helper Functions
fn get_user_balance_internal(user: Principal) -> UserBalance {
    BALANCES.with(|balances| {
        balances.borrow().get(&user).unwrap_or(UserBalance {
            user,
            available: 0,
            locked: 0,
            updated_at: time(),
        })
    })
}

fn lock_funds(user: Principal, amount: Balance) {
    let mut balance = get_user_balance_internal(user);
    balance.available -= amount;
    balance.locked += amount;
    balance.updated_at = time();
    
    BALANCES.with(|balances| balances.borrow_mut().insert(user, balance));
}

fn unlock_funds(user: Principal, amount: Balance) {
    let mut balance = get_user_balance_internal(user);
    balance.locked -= amount;
    balance.available += amount;
    balance.updated_at = time();
    
    BALANCES.with(|balances| balances.borrow_mut().insert(user, balance));
}

fn update_transaction_status(transaction_id: TransactionId, new_status: TransactionStatus) {
    TRANSACTIONS.with(|tx| {
        let mut transactions = tx.borrow_mut();
        if let Some(mut transaction) = transactions.get(&transaction_id) {
            transaction.status = new_status;
            transaction.updated_at = time();
            transactions.insert(transaction_id, transaction);
        }
    });
}