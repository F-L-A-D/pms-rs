use rust_decimal::Decimal;

use uuid::Uuid;

use crate::domain::semantic::settlement_state::SettlementState;

#[derive(Debug, Clone)]
pub struct ReceivableState {
    pub invoice_id: Uuid,

    pub outstanding_amount: Decimal,

    pub settlement_state: SettlementState,
}

impl ReceivableState {
    pub fn is_settled(&self) -> bool {
        self.outstanding_amount <= Decimal::ZERO
    }

    pub fn has_outstanding_balance(&self) -> bool {
        self.outstanding_amount > Decimal::ZERO
    }
}
