use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AllocateExistingPaymentInput {
    pub payment_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
}
