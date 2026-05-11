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