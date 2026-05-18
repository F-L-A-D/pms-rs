use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementEventType {
    InvoiceIssued,
    PaymentApplied,
    RefundApplied,
    WriteoffApplied,
}

impl SettlementEventType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::InvoiceIssued => "invoice_issued",
            Self::PaymentApplied => "payment_applied",
            Self::RefundApplied => "refund_applied",
            Self::WriteoffApplied => "writeoff_applied",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "invoice_issued" => Some(Self::InvoiceIssued),
            "payment_applied" => Some(Self::PaymentApplied),
            "refund_applied" => Some(Self::RefundApplied),
            "writeoff_applied" => Some(Self::WriteoffApplied),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEvent {
    pub id: Uuid,

    pub invoice_id: Uuid,

    pub event_type: SettlementEventType,

    pub amount: Decimal,

    pub occurred_at: DateTime<Utc>,
}
