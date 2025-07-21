use candid::{CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Transaction {
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

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    DirectPayment,
    Escrow {
        release_conditions: Vec<String>,
        auto_release_after: Option<u64>,
    },
    ScheduledPayment {
        schedule: PaymentSchedule,
    },
    Refund {
        original_transaction_id: u64,
    },
    Dispute {
        reason: String,
        evidence: Vec<String>,
    },
    Release,
    Withdrawal,
    Deposit,
    Reversal,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Draft,
    Pending,
    
    Approved,
    Processing,
    InEscrow,
    
    Completed,
    Cancelled {
        reason: String,
        cancelled_by: Principal,
        cancelled_at: u64,
    },
    Failed {
        reason: String,
        failed_at: u64,
    },
    
    Disputed {
        reason: String,
        disputed_by: Principal,
        disputed_at: u64,
    },
    UnderReview {
        reviewer: Principal,
        review_started_at: u64,
    },
    
    Refunded {
        refund_transaction_id: u64,
        refunded_at: u64,
    },
    PartiallyRefunded {
        refunded_amount: u64,
        refund_transaction_ids: Vec<u64>,
    },
    Resolved {
        resolution: DisputeResolution,
        resolved_at: u64,
        resolved_by: Principal,
    },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    ICP,
    Cycles,
    USDT,
    Custom { symbol: String, decimals: u8 },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub struct PaymentSchedule {
    pub frequency: PaymentFrequency,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub amount_per_payment: u64,
    pub total_payments: Option<u32>,
    pub payments_completed: u32,
    pub next_payment_date: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum PaymentFrequency {
    OneTime,
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom { interval_days: u32 },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub invoice_id: Option<String>,
    pub order_id: Option<String>,
    pub category: Option<TransactionCategory>,
    pub tags: Vec<String>,
    pub notes: Vec<TransactionNote>,
    pub attachments: Vec<Attachment>,
    pub custom_fields: Vec<(String, String)>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum TransactionCategory {
    Business,
    Personal,
    Investment,
    Salary,
    Freelance,
    Refund,
    Other { name: String },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TransactionNote {
    pub author: Principal,
    pub content: String,
    pub created_at: u64,
    pub is_private: bool,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: u64,
    pub url: String,
    pub uploaded_by: Principal,
    pub uploaded_at: u64,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub transaction_type: TransactionType,
    pub to: Principal,
    pub amount: u64,
    pub currency: Currency,
    pub description: String,
    pub escrow_agent: Option<Principal>,
    pub deadline: Option<u64>,
    pub category: Option<TransactionCategory>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    pub description: Option<String>,
    pub deadline: Option<u64>,
    pub tags: Option<Vec<String>>,
    pub category: Option<TransactionCategory>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TransactionFilter {
    pub status: Option<Vec<TransactionStatus>>,
    pub transaction_type: Option<Vec<TransactionType>>,
    pub from: Option<Principal>,
    pub to: Option<Principal>,
    pub min_amount: Option<u64>,
    pub max_amount: Option<u64>,
    pub currency: Option<Currency>,
    pub category: Option<TransactionCategory>,
    pub tags: Option<Vec<String>>,
    pub date_range: Option<crate::types::common::TimeFilter>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct TransactionStatistics {
    pub total_transactions: u64,
    pub total_volume: u64,
    pub total_fees: u64,
    pub completed_count: u64,
    pub pending_count: u64,
    pub failed_count: u64,
    pub average_transaction_size: u64,
    pub transactions_by_type: Vec<(TransactionType, u64)>,
    pub transactions_by_status: Vec<(TransactionStatus, u64)>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Balance {
    pub principal: Principal,
    pub currency: Currency,
    pub available: u64,
    pub locked: u64,
    pub pending_incoming: u64,
    pub pending_outgoing: u64,
    pub total_received: u64,
    pub total_sent: u64,
    pub last_transaction_id: Option<u64>,
    pub updated_at: u64,
}

#[derive(Clone, Debug, candid::CandidType, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum DisputeResolution {
    ReleaseToRecipient,
    RefundToSender,
    SplitBetweenParties { sender_percentage: u8 },
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct BalanceHistoryEntry {
    pub principal: Principal,
    pub timestamp: u64,
    pub balance_before: u64,
    pub balance_after: u64,
    pub change: i64,
    pub transaction_id: u64,
    pub transaction_type: TransactionType,
    pub description: String,
}

impl Storable for BalanceHistoryEntry {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).expect("Failed to encode BalanceHistoryEntry"))
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).expect("Failed to decode BalanceHistoryEntry")
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Default for TransactionMetadata {
    fn default() -> Self {
        Self {
            invoice_id: None,
            order_id: None,
            category: None,
            tags: vec![],
            notes: vec![],
            attachments: vec![],
            custom_fields: vec![],
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::ICP
    }
}