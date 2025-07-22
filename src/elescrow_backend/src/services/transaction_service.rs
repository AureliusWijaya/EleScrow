use candid::Principal;
use ic_cdk::api::time;
use std::cell::RefCell;
use crate::types::{
    errors::ApiError,
    transaction::*,
    common::{PaginationParams, AuditAction},
};
use crate::models::transaction::TransactionModel;
use crate::storage::stable_storage::StorageManager;
use crate::security::{
    validation,
    audit::AuditLogger,
};
use crate::services::notification_service::NotificationService;
use crate::BALANCE_SERVICE;

pub struct TransactionService {
    next_id: RefCell<u64>,
    fee_percentage: u64,
    min_transaction_amount: u64,
    max_transaction_amount: u64,
    
    // Dependencies
    audit_logger: RefCell<AuditLogger>,
    notification_service: RefCell<NotificationService>,
}

impl TransactionService {
    pub fn new() -> Self {
        Self {
            next_id: RefCell::new(1),
            fee_percentage: 100, // 1% fee
            min_transaction_amount: 1000,
            max_transaction_amount: 1_000_000_000,
            audit_logger: RefCell::new(AuditLogger::with_defaults()),
            notification_service: RefCell::new(NotificationService::new()),
        }
    }
    
    fn storage(&self) -> &'static StorageManager {
        StorageManager::instance()
    }

    pub fn create_transaction(
        &self,
        from: Principal,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }

        validation::validate_amount(
            request.amount,
            Some(self.min_transaction_amount),
            Some(self.max_transaction_amount),
        )?;
        let description = validation::validate_text(&request.description, "description", 1, 500)?;

        if from == request.to {
            return Err(ApiError::ValidationError {
                field: "to".to_string(),
                message: "Cannot send to yourself".to_string(),
            });
        }

        let balance = BALANCE_SERVICE.with(|s| s.borrow_mut().get_balance(from))?;
        let fee = self.calculate_fee(request.amount);
        let total_amount = request.amount + fee;
        
        if balance.available < total_amount {
            return Err(ApiError::InsufficientFunds {
                available: balance.available,
                required: total_amount,
            });
        }

        let id = self.get_next_id();

        BALANCE_SERVICE.with(|s| {
            s.borrow_mut()
                .lock_funds(from, total_amount, id)
        })?;

        if let Some(deadline) = request.deadline {
            validation::validate_timestamp(deadline, "deadline")?;
            if deadline <= time() {
                return Err(ApiError::ValidationError {
                    field: "deadline".to_string(),
                    message: "Deadline must be in the future".to_string(),
                });
            }
        }
        
        let now = time();
        
        let transaction_model = TransactionModel {
            id,
            transaction_type: request.transaction_type.clone(),
            from,
            to: request.to,
            amount: request.amount,
            fee,
            currency: request.currency,
            description,
            status: TransactionStatus::Pending,
            escrow_agent: request.escrow_agent,
            created_at: now,
            updated_at: now,
            completed_at: None,
            deadline: request.deadline,
            metadata: TransactionMetadata {
                category: request.category,
                tags: request.tags,
                ..Default::default()
            },
        };

        let storage = self.storage();
        storage.transactions().insert(id, transaction_model.clone());
        storage.user_transactions().insert_indexed(id, transaction_model.clone(), from);
        storage.user_transactions().insert_indexed(id, transaction_model.clone(), request.to);

        self.notification_service.borrow().create_transaction_notification(
            request.to,
            id,
            &format!("New transaction from {}", from.to_text()),
        );

        self.audit_logger.borrow().log(
            from,
            AuditAction::TransactionCreated,
            &format!("transaction_{}", id),
            Some(format!("Amount: {}, To: {}", request.amount, request.to)),
        );
        
        Ok(transaction_model.into())
    }

    pub fn approve_transaction(
        &mut self,
        transaction_id: u64,
        approver: Principal,
    ) -> Result<Transaction, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let mut transaction = self.get_transaction_model(transaction_id)?; 

        // if transaction.to != approver {
        //     return Err(ApiError::Unauthorized {
        //         reason: "Only recipient can approve transaction".to_string(),
        //     });
        // }

        if !matches!(transaction.status, TransactionStatus::Pending) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", transaction.status),
                required_state: "Pending".to_string(),
            });
        }

        transaction.status = TransactionStatus::Approved;
        transaction.updated_at = time();
        
        self.storage().transactions().insert(transaction_id, transaction.clone());

        self.notification_service.borrow().create_transaction_notification(
            transaction.from,
            transaction_id,
            "Transaction approved",
        );

        self.audit_logger.borrow().log(
            approver,
            AuditAction::TransactionApproved,
            &format!("transaction_{}", transaction_id),
            None,
        );
        
        Ok(transaction.into())
    }

    pub fn accept_escrow_terms(
        &mut self,
        transaction_id: u64,
        acceptor: Principal,
    ) -> Result<Transaction, ApiError> {
        let mut transaction = self.get_transaction_model(transaction_id)?;

        if transaction.to != acceptor {
            return Err(ApiError::Unauthorized {
                reason: "Only the recipient (User B) can accept the escrow terms.".to_string(),
            });
        }

        if !matches!(transaction.status, TransactionStatus::Pending) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", transaction.status),
                required_state: "Pending".to_string(),
            });
        }

        transaction.status = TransactionStatus::InEscrow;
        transaction.updated_at = time();
        
        self.storage().transactions().insert(transaction_id, transaction.clone());

        let _ = self.notification_service.borrow().create_transaction_notification(
            transaction.from,
            transaction_id,
            "Escrow terms have been accepted.",
        );

        Ok(transaction.into())
    }

    pub fn submit_escrow_work(
        &mut self,
        transaction_id: u64,
        submitter: Principal,
    ) -> Result<Transaction, ApiError> {
        let mut transaction = self.get_transaction_model(transaction_id)?;

        if transaction.to != submitter {
            return Err(ApiError::Unauthorized {
                reason: "Only the recipient (User B) can submit work for this escrow.".to_string(),
            });
        }

        if !matches!(transaction.status, TransactionStatus::InEscrow) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", transaction.status),
                required_state: "InEscrow".to_string(),
            });
        }

        transaction.status = TransactionStatus::SubmittedForReview {
            submitted_at: time(),
        };
        transaction.updated_at = time();
        
        self.storage().transactions().insert(transaction_id, transaction.clone());

        let _ = self.notification_service.borrow().create_transaction_notification(
            transaction.from,
            transaction_id,
            "Work has been submitted for your review.",
        );

        Ok(transaction.into())
    }
    
    pub fn complete_transaction(
        &mut self,
        transaction_id: u64,
        completer: Principal,
    ) -> Result<Transaction, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let mut transaction = self.get_transaction_model(transaction_id)?;

        let authorized = transaction.from == completer ||
                        transaction.to == completer ||
                        transaction.escrow_agent == Some(completer);
        
        if transaction.from != completer {
            return Err(ApiError::Unauthorized {
                reason: "Only the sender (User A) can release the funds for this escrow.".to_string(),
            });
        }

        if !matches!(transaction.status, TransactionStatus::SubmittedForReview { .. }) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", transaction.status),
                required_state: "SubmittedForReview".to_string(),
            });
        }

        let total_amount = transaction.amount + transaction.fee;
        BALANCE_SERVICE.with(|s| {
            s.borrow_mut().transfer_locked_funds(
                transaction.from,
                transaction.to,
                transaction.amount,
                transaction_id,
                "Transaction completed",
            )
        })?;
        
        transaction.status = TransactionStatus::Completed;
        transaction.completed_at = Some(time());
        transaction.updated_at = time();
        
        self.storage().transactions().insert(transaction_id, transaction.clone());
        
        // self.update_balance_statistics(&transaction);
        
        self.notification_service.borrow().create_transaction_notification(
            transaction.from,
            transaction_id,
            "Transaction completed",
        );
        self.notification_service.borrow().create_transaction_notification(
            transaction.to,
            transaction_id,
            &format!("Payment received: {} {:?}", transaction.amount, transaction.currency),
        );
        
        self.audit_logger.borrow().log(
            completer,
            AuditAction::TransactionCompleted,
            &format!("transaction_{}", transaction_id),
            None,
        );
        
        Ok(transaction.into())
    }
    
    pub fn cancel_transaction(
        &self,
        transaction_id: u64,
        canceller: Principal,
        reason: String,
    ) -> Result<Transaction, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let mut transaction = self.get_transaction_model(transaction_id)?;
        
        if transaction.from != canceller {
            return Err(ApiError::Unauthorized {
                reason: "Only sender can cancel transaction".to_string(),
            });
        }
        
        if !matches!(transaction.status, TransactionStatus::Pending) && !matches!(transaction.status, TransactionStatus::Approved) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", transaction.status),
                required_state: "Pending or Approved".to_string(),
            });
        }

        let total_amount = transaction.amount + transaction.fee;
        BALANCE_SERVICE.with(|s| {
            s.borrow_mut()
                .unlock_funds(transaction.from, total_amount, transaction_id)
        })?;

        transaction.status = TransactionStatus::Cancelled {
            reason: reason.clone(),
            cancelled_by: canceller,
            cancelled_at: time(),
        };
        transaction.updated_at = time();
        
        self.storage().transactions().insert(transaction_id, transaction.clone());

        self.notification_service.borrow().create_transaction_notification(
            transaction.to,
            transaction_id,
            &format!("Transaction cancelled: {}", reason),
        );

        self.audit_logger.borrow().log(
            canceller,
            AuditAction::TransactionCancelled,
            &format!("transaction_{}", transaction_id),
            Some(reason),
        );
        
        Ok(transaction.into())
    }

    pub fn get_transaction(&self, transaction_id: u64, requester: Principal) -> Result<Transaction, ApiError> {
        let transaction = self.get_transaction_model(transaction_id)?;

        if transaction.from != requester &&
           transaction.to != requester &&
           transaction.escrow_agent != Some(requester) {
            return Err(ApiError::Unauthorized {
                reason: "Not authorized to view this transaction".to_string(),
            });
        }
        
        Ok(transaction.into())
    }

    pub fn get_user_transactions(
        &self,
        user: Principal,
        filter: Option<TransactionFilter>,
        pagination: PaginationParams,
    ) -> Result<Vec<Transaction>, ApiError> {
        pagination.validate()?;
        
        let transactions: Vec<Transaction> = self.storage().transactions()
            .filter(|_, tx| {
                if tx.from != user && tx.to != user {
                    return false;
                }
                
                if let Some(ref f) = filter {
                    if let Some(ref statuses) = f.status {
                        if !statuses.iter().any(|s| std::mem::discriminant(s) == std::mem::discriminant(&tx.status)) {
                            return false;
                        }
                    }
                    
                    if let Some(min) = f.min_amount {
                        if tx.amount < min {
                            return false;
                        }
                    }
                    
                    if let Some(max) = f.max_amount {
                        if tx.amount > max {
                            return false;
                        }
                    }
                }
                
                true
            })
            .into_iter()
            .map(|(_, tx)| tx.into())
            .skip(pagination.offset as usize)
            .take(pagination.limit as usize)
            .collect();
        
        Ok(transactions)
    }
    
    // fn credit_funds(&self, principal: Principal, amount: u64) -> Result<(), ApiError> {
    //     let mut balance = self.get_or_create_balance(principal);
        
    //     balance.available += amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance);
    //     Ok(())
    // }
    
    // fn update_balance_statistics(&self, transaction: &TransactionModel) {
    //     if let Some(mut balance) = self.storage().balances().get(&transaction.from) {
    //         balance.total_sent += transaction.amount;
    //         balance.last_transaction_id = Some(transaction.id);
    //         self.storage().balances().insert(transaction.from, balance);
    //     }
        
    //     if let Some(mut balance) = self.storage().balances().get(&transaction.to) {
    //         balance.total_received += transaction.amount;
    //         balance.last_transaction_id = Some(transaction.id);
    //         self.storage().balances().insert(transaction.to, balance);
    //     }
    // }
    
    // pub fn get_balance(&self, principal: Principal) -> Balance {
    //     self.get_or_create_balance(principal)
    // }
    
    // pub fn deposit(&self, principal: Principal, amount: u64) -> Result<Balance, ApiError> {
    //     validation::validate_amount(amount, Some(self.min_transaction_amount), None)?;
        
    //     let mut balance = self.get_or_create_balance(principal);
    //     balance.available += amount;
    //     balance.total_received += amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance.clone());
        
    //     self.audit_logger.borrow().log(
    //         principal,
    //         AuditAction::Deposit,
    //         &principal.to_text(),
    //         Some(format!("Amount: {}", amount)),
    //     );
        
    //     Ok(balance)
    // }
    
    // pub fn withdraw(&self, principal: Principal, amount: u64) -> Result<Balance, ApiError> {
    //     validation::validate_amount(amount, Some(self.min_transaction_amount), None)?;
        
    //     let mut balance = self.get_or_create_balance(principal);
        
    //     if balance.available < amount {
    //         return Err(ApiError::InsufficientFunds {
    //             available: balance.available,
    //             required: amount,
    //         });
    //     }
        
    //     balance.available -= amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance.clone());
        
    //     self.audit_logger.borrow().log(
    //         principal,
    //         AuditAction::Withdrawal,
    //         &principal.to_text(),
    //         Some(format!("Amount: {}", amount)),
    //     );
        
    //     Ok(balance)
    // }

    // pub fn resolve_dispute(
    //     &self,
    //     transaction_id: u64,
    //     resolution: DisputeResolution,
    //     admin_principal: Principal
    // ) -> Result<Transaction, ApiError> {
    //     if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
    //         return Err(ApiError::SystemPaused {
    //             reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
    //         });
    //     }
    //     let mut transaction = self.get_transaction_model(transaction_id)?;

    //     if !matches!(transaction.status, TransactionStatus::Disputed { .. }) {
    //         return Err(ApiError::InvalidState {
    //             current_state: format!("{:?}", transaction.status),
    //             required_state: "Disputed".to_string(),
    //         });
    //     }

    //     match resolution {
    //         DisputeResolution::ReleaseToRecipient => {
    //             self.credit_funds(transaction.to, transaction.amount)?;
    //         }
    //         DisputeResolution::RefundToSender => {
    //             self.credit_funds(transaction.from, transaction.amount)?;
    //         }
    //         DisputeResolution::SplitBetweenParties { sender_percentage } => {
    //             let recipient_percentage = 100 - sender_percentage;
    //             let recipient_amount = transaction.amount * recipient_percentage as u64 / 100;
    //             let sender_amount = transaction.amount - recipient_amount;

    //             self.credit_funds(transaction.to, recipient_amount)?;
    //             self.credit_funds(transaction.from, sender_amount)?;
    //         }
    //     }

    //     transaction.status = TransactionStatus::Resolved {
    //         resolution: resolution.clone(),
    //         resolved_by: admin_principal,
    //         resolved_at: time()
    //     };
    //     transaction.updated_at = time();

    //     self.storage().transactions().insert(transaction_id, transaction.clone());

    //     Ok(transaction.into())
    // }
    
    pub fn reverse_transaction(
        &self,
        transaction_id: u64,
        admin_principal: Principal,
        reason: String,
    ) -> Result<Transaction, ApiError> {
        if crate::SYSTEM_STATE.with(|s| s.borrow().is_paused) {
            return Err(ApiError::SystemPaused {
                reason: crate::SYSTEM_STATE.with(|s| s.borrow().reason.clone().unwrap_or_default()),
            });
        }
        let original_transaction = self.get_transaction_model(transaction_id)?;

        if !matches!(original_transaction.status, TransactionStatus::Completed { .. }) {
            return Err(ApiError::InvalidState {
                current_state: format!("{:?}", original_transaction.status),
                required_state: "Completed".to_string(),
            });
        }

        let reversal_request = CreateTransactionRequest {
            transaction_type: TransactionType::Reversal,
            to: original_transaction.from,
            amount: original_transaction.amount,
            currency: original_transaction.currency,
            description: format!("Reversal of transaction {}: {}", transaction_id, reason),
            escrow_agent: None,
            deadline: None,
            category: None,
            tags: vec!["reversal".to_string()],
        };

        let reversal_transaction = self.create_transaction(original_transaction.to, reversal_request)?;

        self.audit_logger.borrow().log(
            admin_principal,
            AuditAction::TransactionReversed,
            &transaction_id.to_string(),
            Some(reason),
        );

        Ok(reversal_transaction)
    }

    pub fn update_fee_percentage(&mut self, new_fee_bps: u64, admin_principal: Principal) -> Result<(), ApiError> {
        if new_fee_bps > 10000 { // Max 100%
            return Err(ApiError::ValidationError {
                field: "new_fee_bps".to_string(),
                message: "Fee cannot exceed 10000 bps (100%)".to_string(),
            });
        }

        let old_fee = self.fee_percentage;
        self.fee_percentage = new_fee_bps;

        self.audit_logger.borrow().log(
            admin_principal,
            AuditAction::ConfigurationChanged,
            "fee_percentage",
            Some(format!("Changed from {} to {} bps", old_fee, new_fee_bps)),
        );

        Ok(())
    }
    
    fn get_next_id(&self) -> u64 {
        let mut id = self.next_id.borrow_mut();
        let current = *id;
        *id += 1;
        current
    }
    
    fn calculate_fee(&self, amount: u64) -> u64 {
        (amount * self.fee_percentage) / 10_000
    }
    
    fn get_transaction_model(&self, id: u64) -> Result<TransactionModel, ApiError> {
        self.storage().transactions().get_or_error(&id, &format!("Transaction {}", id))
    }
    
    // fn get_or_create_balance(&self, principal: Principal) -> Balance {
    //     self.storage().balances().get(&principal).unwrap_or_else(|| Balance {
    //         principal,
    //         currency: Currency::ICP,
    //         available: 0,
    //         locked: 0,
    //         pending_incoming: 0,
    //         pending_outgoing: 0,
    //         total_received: 0,
    //         total_sent: 0,
    //         last_transaction_id: None,
    //         updated_at: time(),
    //     })
    // }
    
    // fn lock_funds(&self, principal: Principal, amount: u64) -> Result<(), ApiError> {
    //     let mut balance = self.get_or_create_balance(principal);
        
    //     if balance.available < amount {
    //         return Err(ApiError::InsufficientFunds {
    //             available: balance.available,
    //             required: amount,
    //         });
    //     }
        
    //     balance.available -= amount;
    //     balance.locked += amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance);
    //     Ok(())
    // }
    
    // fn unlock_funds(&self, principal: Principal, amount: u64) -> Result<(), ApiError> {
    //     let mut balance = self.get_or_create_balance(principal);
        
    //     if balance.locked < amount {
    //         return Err(ApiError::InternalError {
    //             details: "Insufficient locked funds".to_string(),
    //         });
    //     }
        
    //     balance.locked -= amount;
    //     balance.available += amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance);
    //     Ok(())
    // }
    
    // fn debit_funds(&self, principal: Principal, amount: u64) -> Result<(), ApiError> {
    //     let mut balance = self.get_or_create_balance(principal);
        
    //     if balance.available < amount {
    //         return Err(ApiError::InsufficientFunds {
    //             available: balance.available,
    //             required: amount,
    //         });
    //     }
        
    //     balance.available -= amount;
    //     balance.updated_at = time();
        
    //     self.storage().balances().insert(principal, balance);
    //     Ok(())
    // }
}