use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ApplyDepositToReceivableInput {
    pub deposit_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
}
