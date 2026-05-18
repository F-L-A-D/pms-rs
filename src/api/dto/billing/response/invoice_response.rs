use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::invoice::InvoiceStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub billing_account_id: Uuid,
    pub invoice_number: String,
    pub issued_amount: Decimal,
    pub status: InvoiceStatus,
    pub issued_at: DateTime<Utc>,
}
