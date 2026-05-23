use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementTransitionType {
    InvoiceIssued,
    ReceivableOpened,
    PaymentAllocated,
    ReceivableSettled,
    ReceivableDisputed,
    ReceivableDisputeResolved,
    ReceivableWrittenOff,
    PaymentAllocationReversed,
    InvoiceVoided,
    ReceivableVoided,
    DepositApplied,
}

impl SettlementTransitionType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::InvoiceIssued => "invoice_issued",
            Self::ReceivableOpened => "receivable_opened",
            Self::PaymentAllocated => "payment_allocated",
            Self::ReceivableSettled => "receivable_settled",
            Self::ReceivableDisputed => "receivable_disputed",
            Self::ReceivableDisputeResolved => "receivable_dispute_resolved",
            Self::ReceivableWrittenOff => "receivable_written_off",
            Self::PaymentAllocationReversed => "payment_allocation_reversed",
            Self::InvoiceVoided => "invoice_voided",
            Self::ReceivableVoided => "receivable_voided",
            Self::DepositApplied => "deposit_applied",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "invoice_issued" => Some(Self::InvoiceIssued),
            "receivable_opened" => Some(Self::ReceivableOpened),
            "payment_allocated" => Some(Self::PaymentAllocated),
            "receivable_settled" => Some(Self::ReceivableSettled),
            "receivable_disputed" => Some(Self::ReceivableDisputed),
            "receivable_dispute_resolved" => Some(Self::ReceivableDisputeResolved),
            "receivable_written_off" => Some(Self::ReceivableWrittenOff),
            "payment_allocation_reversed" => Some(Self::PaymentAllocationReversed),
            "invoice_voided" => Some(Self::InvoiceVoided),
            "receivable_voided" => Some(Self::ReceivableVoided),
            "deposit_applied" => Some(Self::DepositApplied),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementTransition {
    pub id: Uuid,
    pub receivable_id: Uuid,
    pub transition_type: SettlementTransitionType,
    pub amount: Decimal,
    pub occurred_at: DateTime<Utc>,
}
