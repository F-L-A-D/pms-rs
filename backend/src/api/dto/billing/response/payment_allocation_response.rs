use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocationResponse {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub allocated_at: DateTime<Utc>,
    pub remaining_outstanding_amount: Decimal,
}
