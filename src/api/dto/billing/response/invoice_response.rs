use chrono::{DateTime, NaiveDate, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::invoice::{Invoice, InvoiceStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub billing_account_id: Uuid,
    pub invoice_number: String,
    pub issued_amount: Decimal,
    pub due_date: NaiveDate,
    pub status: InvoiceStatus,
    pub issued_at: DateTime<Utc>,
}

impl From<Invoice> for InvoiceResponse {
    fn from(invoice: Invoice) -> Self {
        Self {
            id: invoice.id,
            folio_id: invoice.folio_id,
            billing_account_id: invoice.billing_account_id,
            invoice_number: invoice.invoice_number,
            issued_amount: invoice.issued_amount,
            due_date: invoice.due_date,
            status: invoice.status,
            issued_at: invoice.issued_at,
        }
    }
}
