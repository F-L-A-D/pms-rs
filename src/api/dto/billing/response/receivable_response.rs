use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::receivable::{Receivable, ReceivableStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivableResponse {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub outstanding_amount: Decimal,
    pub due_date: NaiveDate,
    pub status: ReceivableStatus,
}

impl From<Receivable> for ReceivableResponse {
    fn from(receivable: Receivable) -> Self {
        Self {
            id: receivable.id,
            invoice_id: receivable.invoice_id,
            outstanding_amount: receivable.outstanding_amount,
            due_date: receivable.due_date,
            status: receivable.status,
        }
    }
}
