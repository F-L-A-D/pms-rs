use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub folio_id: String,
    pub invoice_number: String,
    pub issued_amount: String,
    pub due_date: chrono::NaiveDate,
}
