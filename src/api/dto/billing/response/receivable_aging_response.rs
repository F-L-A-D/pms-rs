use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivableAgingResponse {
    pub current_amount: Decimal,
    pub overdue_1_30_amount: Decimal,
    pub overdue_31_60_amount: Decimal,
    pub overdue_61_90_amount: Decimal,
    pub overdue_90_plus_amount: Decimal,
    pub total_open_amount: Decimal,
}
