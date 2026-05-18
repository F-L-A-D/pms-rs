use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateInvoiceInput {
    pub folio_id: Uuid,
    pub invoice_number: String,
    pub issued_amount: Decimal,
}
