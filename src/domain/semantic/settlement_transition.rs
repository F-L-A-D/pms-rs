use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettlementTransitionType {
    InvoiceIssued,
    ReceivableOpened,
}

impl SettlementTransitionType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::InvoiceIssued => "invoice_issued",
            Self::ReceivableOpened => "receivable_opened",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "invoice_issued" => Some(Self::InvoiceIssued),
            "receivable_opened" => Some(Self::ReceivableOpened),
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
