use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_stable_structures::Storable;
use std::borrow::Cow;
use crate::types::transaction::*;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TransactionModel {
    pub id: u64,
    pub transaction_type: TransactionType,
    pub from: Principal,
    pub to: Principal,
    pub amount: u64,
    pub fee: u64,
    pub currency: Currency,
    pub description: String,
    pub status: TransactionStatus,
    pub escrow_agent: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
    pub completed_at: Option<u64>,
    pub deadline: Option<u64>,
    pub metadata: TransactionMetadata,
}

impl Storable for TransactionModel {
    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 4096,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = serde_cbor::to_vec(self).expect("Failed to serialize TransactionModel");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).expect("Failed to deserialize TransactionModel")
    }
}

impl From<TransactionModel> for Transaction {
    fn from(model: TransactionModel) -> Self {
        Transaction {
            id: model.id,
            transaction_type: model.transaction_type,
            from: model.from,
            to: model.to,
            amount: model.amount,
            fee: model.fee,
            currency: model.currency,
            description: model.description,
            status: model.status,
            escrow_agent: model.escrow_agent,
            created_at: model.created_at,
            updated_at: model.updated_at,
            completed_at: model.completed_at,
            deadline: model.deadline,
            metadata: model.metadata,
        }
    }
}

impl From<Transaction> for TransactionModel {
    fn from(tx: Transaction) -> Self {
        TransactionModel {
            id: tx.id,
            transaction_type: tx.transaction_type,
            from: tx.from,
            to: tx.to,
            amount: tx.amount,
            fee: tx.fee,
            currency: tx.currency,
            description: tx.description,
            status: tx.status,
            escrow_agent: tx.escrow_agent,
            created_at: tx.created_at,
            updated_at: tx.updated_at,
            completed_at: tx.completed_at,
            deadline: tx.deadline,
            metadata: tx.metadata,
        }
    }
}