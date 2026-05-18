use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::receivable::ReceivableStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivableResponse {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub outstanding_amount: Decimal,
    pub due_date: NaiveDate,
    pub status: ReceivableStatus,
}
