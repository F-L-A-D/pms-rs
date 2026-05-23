use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefundDepositInput {
    pub deposit_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
}
