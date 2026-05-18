use rust_decimal::Decimal;

use uuid::Uuid;

use crate::domain::semantic::settlement_state::SettlementState;

#[derive(Debug, Clone)]
pub struct InvoiceSettlement {
    pub invoice_id: Uuid,

    pub settlement_state: SettlementState,

    pub settled_amount: Decimal,
}
