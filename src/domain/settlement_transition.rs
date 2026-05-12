use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum SettlementTransitionType {
    InvoiceIssued,
    ReceivableOpened,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettlementTransition {
    pub id: Uuid,
    pub receivable_id: Uuid,
    pub transition_type: SettlementTransitionType,
    pub amount: i64,
    pub occurred_at: DateTime<Utc>,
}

impl SettlementTransitionType {

    pub fn as_str(
        &self,
    ) -> &'static str {

        match self {

            Self::InvoiceIssued =>
                "InvoiceIssued",

            Self::ReceivableOpened =>
                "ReceivableOpened",
        }
    }

    pub fn from_str(
        value: &str,
    ) -> Result<Self, String> {

        match value {

            "InvoiceIssued" =>
                Ok(Self::InvoiceIssued),

            "ReceivableOpened" =>
                Ok(Self::ReceivableOpened),

            _ => {
                Err(
                    format!(
                        "invalid settlement transition type: {}",
                        value,
                    )
                )
            }
        }
    }
}