use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefundPaymentInput {
    pub payment_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
}