use candid::Principal;
use ic_cdk::api::time;

use crate::types::{
    errors::ApiError,
    transaction::{Balance, BalanceHistoryEntry, Currency},
    common::PaginationParams,
};
use crate::storage::{
    stable_storage::{StableStorage, TimeSeriesStorage},
    memory::MemoryRegion,
};
use crate::security::validation;

pub struct BalanceService {
    balances: StableStorage<Principal, Balance>,
    balance_history: TimeSeriesStorage<BalanceHistoryEntry>,
    
    min_balance: u64,
    max_balance: u64,
}

impl BalanceService {
    pub fn new() -> Self {
        Self {
            balances: StableStorage::new(MemoryRegion::Balances),
            balance_history: TimeSeriesStorage::new(MemoryRegion::BalanceHistory),
            min_balance: 0,
            max_balance: u64::MAX,
        }
    }
    
    pub fn get_or_create_balance(&mut self, principal: Principal) -> Balance {
        self.balances.get(&principal).unwrap_or_else(|| Balance {
            principal,
            currency: Currency::ICP,
            available: 0,
            locked: 0,
            pending_incoming: 0,
            pending_outgoing: 0,
            total_received: 0,
            total_sent: 0,
            last_transaction_id: None,
            updated_at: time(),
        })
    }
    
    pub fn get_balance(&mut self, principal: Principal) -> Result<Balance, ApiError> {
        validation::validate_principal(&principal)?;
        Ok(self.get_or_create_balance(principal))
    }

    pub fn deposit(&mut self, principal: Principal, amount: u64) -> Result<u64, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        validation::validate_principal(&principal)?;
        let mut balance = self.get_or_create_balance(principal);
        let balance_before = balance.available;

        balance.available = balance.available.checked_add(amount)
            .ok_or_else(|| ApiError::InternalError {
                details: "Balance overflow".to_string(),
            })?;
        
        balance.total_received = balance.total_received.saturating_add(amount);
        balance.last_transaction_id = None;
        balance.updated_at = time();
        
        self.balances.insert(principal, balance.clone());
        
        self.record_history(
            principal,
            balance_before,
            balance.available,
            amount as i64,
            0,
            "Deposit",
        );
        
        Ok(balance.available)
    }

    pub fn withdraw(&mut self, principal: Principal, amount: u64) -> Result<u64, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        validation::validate_principal(&principal)?;
        let mut balance = self.get_or_create_balance(principal);
        let balance_before = balance.available;
        
        if balance.available < amount {
            return Err(ApiError::InsufficientFunds {
                available: balance.available,
                required: amount,
            });
        }

        balance.available -= amount;
        balance.total_sent = balance.total_sent.saturating_add(amount);
        balance.last_transaction_id = None;
        balance.updated_at = time();

        self.balances.insert(principal, balance.clone());

        self.record_history(
            principal,
            balance_before,
            balance.available,
            -(amount as i64),
            0,
            "Withdrawal",
        );
        
        Ok(balance.available)
    }
    
    pub fn credit_funds(
        &mut self,
        principal: Principal,
        amount: u64,
        transaction_id: u64,
        description: &str,
    ) -> Result<Balance, ApiError> {
        validation::validate_amount(amount, Some(1), None)?;
        
        let mut balance = self.get_or_create_balance(principal);
        let balance_before = balance.available;
        
        balance.available = balance.available.checked_add(amount)
            .ok_or_else(|| ApiError::InternalError {
                details: "Balance overflow".to_string(),
            })?;
        
        balance.total_received = balance.total_received.saturating_add(amount);
        balance.last_transaction_id = Some(transaction_id);
        balance.updated_at = time();
        
        self.balances.insert(principal, balance.clone());
        
        self.record_history(
            principal,
            balance_before,
            balance.available,
            amount as i64,
            transaction_id,
            description,
        );
        
        Ok(balance)
    }
    
    pub fn debit_funds(
        &mut self,
        principal: Principal,
        amount: u64,
        transaction_id: u64,
        description: &str,
    ) -> Result<Balance, ApiError> {
        validation::validate_amount(amount, Some(1), None)?;
        
        let mut balance = self.get_or_create_balance(principal);
        let balance_before = balance.available;

        if balance.available < amount {
            return Err(ApiError::InsufficientFunds {
                available: balance.available,
                required: amount,
            });
        }

        balance.available -= amount;
        balance.total_sent = balance.total_sent.saturating_add(amount);
        balance.last_transaction_id = Some(transaction_id);
        balance.updated_at = time();

        self.balances.insert(principal, balance.clone());

        self.record_history(
            principal,
            balance_before,
            balance.available,
            -(amount as i64),
            transaction_id,
            description,
        );
        
        Ok(balance)
    }
    
    pub fn lock_funds(
        &mut self,
        principal: Principal,
        amount: u64,
        transaction_id: u64,
    ) -> Result<Balance, ApiError> {
        validation::validate_amount(amount, Some(1), None)?;
        
        let mut balance = self.get_or_create_balance(principal);

        if balance.available < amount {
            return Err(ApiError::InsufficientFunds {
                available: balance.available,
                required: amount,
            });
        }

        balance.available -= amount;
        balance.locked = balance.locked.checked_add(amount)
            .ok_or_else(|| ApiError::InternalError {
                details: "Locked balance overflow".to_string(),
            })?;
        
        balance.last_transaction_id = Some(transaction_id);
        balance.updated_at = time();
        
        self.balances.insert(principal, balance.clone());
        
        Ok(balance)
    }

    pub fn unlock_funds(
        &mut self,
        principal: Principal,
        amount: u64,
        transaction_id: u64,
    ) -> Result<Balance, ApiError> {
        validation::validate_amount(amount, Some(1), None)?;
        
        let mut balance = self.get_or_create_balance(principal);

        if balance.locked < amount {
            return Err(ApiError::InternalError {
                details: "Insufficient locked funds".to_string(),
            });
        }

        balance.locked -= amount;
        balance.available = balance.available.checked_add(amount)
            .ok_or_else(|| ApiError::InternalError {
                details: "Available balance overflow".to_string(),
            })?;
        
        balance.last_transaction_id = Some(transaction_id);
        balance.updated_at = time();
        
        self.balances.insert(principal, balance.clone());
        
        Ok(balance)
    }

    pub fn transfer_locked_funds(
        &mut self,
        from: Principal,
        to: Principal,
        amount: u64,
        transaction_id: u64,
        description: &str,
    ) -> Result<(Balance, Balance), ApiError> {
        validation::validate_amount(amount, Some(1), None)?;

        let mut from_balance = self.get_or_create_balance(from);
        
        if from_balance.locked < amount {
            return Err(ApiError::InternalError {
                details: "Insufficient locked funds".to_string(),
            });
        }
        
        from_balance.locked -= amount;
        from_balance.total_sent = from_balance.total_sent.saturating_add(amount);
        from_balance.last_transaction_id = Some(transaction_id);
        from_balance.updated_at = time();

        let mut to_balance = self.get_or_create_balance(to);
        let to_balance_before = to_balance.available;
        
        to_balance.available = to_balance.available.checked_add(amount)
            .ok_or_else(|| ApiError::InternalError {
                details: "Balance overflow".to_string(),
            })?;
        
        to_balance.total_received = to_balance.total_received.saturating_add(amount);
        to_balance.last_transaction_id = Some(transaction_id);
        to_balance.updated_at = time();

        self.balances.insert(from, from_balance.clone());
        self.balances.insert(to, to_balance.clone());

        self.record_history(
            to,
            to_balance_before,
            to_balance.available,
            amount as i64,
            transaction_id,
            description,
        );
        
        Ok((from_balance, to_balance))
    }

    pub fn get_balance_history(
        &self,
        principal: Principal,
        start_date: Option<u64>,
        end_date: Option<u64>,
        pagination: PaginationParams,
    ) -> Result<Vec<BalanceHistoryEntry>, ApiError> {
        pagination.validate()?;
        
        let start = start_date.unwrap_or(0);
        let end = end_date.unwrap_or(time());
        
        let history: Vec<BalanceHistoryEntry> = self.balance_history
            .range(start, end)
            .into_iter()
            .map(|(_, entry)| entry)
            .filter(|entry| entry.principal == principal)
            .skip(pagination.offset as usize)
            .take(pagination.limit as usize)
            .collect();
        
        Ok(history)
    }
    
    pub fn get_total_statistics(&self) -> BalanceStatistics {
        let mut stats = BalanceStatistics {
            total_users: 0,
            total_available: 0,
            total_locked: 0,
            total_pending_incoming: 0,
            total_pending_outgoing: 0,
            total_volume: 0,
        };
        
        for (_, balance) in self.balances.entries() {
            stats.total_users += 1;
            stats.total_available += balance.available;
            stats.total_locked += balance.locked;
            stats.total_pending_incoming += balance.pending_incoming;
            stats.total_pending_outgoing += balance.pending_outgoing;
            stats.total_volume += balance.total_sent + balance.total_received;
        }
        
        stats
    }

    fn record_history(
        &mut self,
        principal: Principal,
        balance_before: u64,
        balance_after: u64,
        change: i64,
        transaction_id: u64,
        description: &str,
    ) {
        let entry = BalanceHistoryEntry {
            principal,
            timestamp: time(),
            balance_before,
            balance_after,
            change,
            transaction_id,
            transaction_type: crate::types::transaction::TransactionType::DirectPayment,
            description: description.to_string(),
        };
        
        self.balance_history.add(time(), entry);
    }
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize)]
pub struct BalanceStatistics {
    pub total_users: u64,
    pub total_available: u64,
    pub total_locked: u64,
    pub total_pending_incoming: u64,
    pub total_pending_outgoing: u64,
    pub total_volume: u64,
}