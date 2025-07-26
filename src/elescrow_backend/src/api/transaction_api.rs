use std::future;

use candid::{candid_method, Principal};
use ic_cdk_macros::{query, update};
use ic_cdk::api::caller;
use crate::services::balance_service::BalanceService;
use crate::types::{
    errors::ApiError,
    transaction::*,
    common::PaginationParams,
};
use crate::{TRANSACTION_SERVICE, BALANCE_SERVICE};

use candid::Nat;
use num_traits::ToPrimitive;
use icrc_ledger_types::icrc1::account::Account;


#[update]
#[candid_method(update)]
pub async fn create_transaction(request: CreateTransactionRequest) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().create_transaction(caller, request)
    })
}

#[update]
#[candid_method(update)]
pub fn accept_escrow_terms(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().accept_escrow_terms(transaction_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn submit_escrow_work(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().submit_escrow_work(transaction_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn approve_transaction(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().approve_transaction(transaction_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn complete_transaction(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().complete_transaction(transaction_id, caller)
    })
}

#[update]
#[candid_method(update)]
pub fn raise_dispute(transaction_id: u64, reason: String) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow_mut().raise_dispute(transaction_id, caller, reason)
    })
}

#[update]
#[candid_method(update)]
pub fn cancel_transaction(transaction_id: u64, reason: String) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().cancel_transaction(transaction_id, caller, reason)
    })
}

#[query]
#[candid_method(query)]
pub fn get_transaction(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().get_transaction(transaction_id, caller)
    })
}

#[query]
#[candid_method(query)]
pub fn get_my_transactions(
    filter: Option<TransactionFilter>,
    pagination: PaginationParams,
) -> Result<Vec<Transaction>, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().get_user_transactions(caller, filter, pagination)
    })
}

#[update]
#[candid_method(update)]
pub async fn get_balance() -> Result<Balance, ApiError> {
    let caller = caller();
    BalanceService::check_ledger_balance(caller).await
}

#[update]
#[candid_method(update)]
pub async fn deposit(amount: u64) -> Result<u64, ApiError> {
    let caller = caller();
    
    BALANCE_SERVICE.with(|service| {
        service.borrow_mut().deposit(caller, amount)
    })
}

#[update]
#[candid_method(update)]
pub async fn withdraw(amount: u64) -> Result<u64, ApiError> {
    let caller = caller();
    
    BALANCE_SERVICE.with(|service| {
        service.borrow_mut().withdraw(caller, amount)
    })
}

#[query(composite = true)]
#[candid_method(query)]
pub async fn query_ledger_balance(account_owner: Principal) -> Result<u64, String> {
    let account = Account {
        owner: account_owner,
        subaccount: None,
    };
    let args = (account,);

    let ledger_canister_id = BalanceService::ledger_canister_id();

    let call_result: Result<(Nat,), _> = ic_cdk::call(
        ledger_canister_id,
        "icrc1_balance_of",
        args,
    )
    .await;

    match call_result {
        Ok((nat_balance,)) => {
            let balance_u64 = nat_balance.0.to_u64().ok_or_else(|| {
                format!("Ledger balance ({}) is too large to fit in a u64.", nat_balance)
            })?;
            Ok(balance_u64)
        }
        Err((code, msg)) => {
            Err(format!("Ledger query failed: [{:?}] {}", code, msg))
        }
    }
}
#[update]
#[candid_method(update)]
pub fn create_scheduled_payment(
    to: candid::Principal,
    amount: u64,
    schedule: PaymentSchedule,
    description: String,
) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    let request = CreateTransactionRequest {
        transaction_type: TransactionType::ScheduledPayment { schedule },
        to,
        amount,
        currency: Currency::ICP,
        description,
        escrow_agent: None,
        deadline: None,
        category: None,
        tags: vec![],
    };
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().create_transaction(caller, request)
    })
}

#[update]
#[candid_method(update)]
pub fn cancel_scheduled_payment(transaction_id: u64) -> Result<Transaction, ApiError> {
    let caller = caller();
    
    TRANSACTION_SERVICE.with(|service| {
        service.borrow().cancel_transaction(transaction_id, caller, "User cancelled scheduled payment".to_string())
    })
}