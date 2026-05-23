use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::Serialize;

use uuid::Uuid;

use crate::domain::entity::deposit_refund::DepositRefund;

#[derive(Debug, Serialize)]
pub struct DepositRefundResponse {
    pub id: Uuid,
    pub deposit_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refunded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<DepositRefund> for DepositRefundResponse {
    fn from(value: DepositRefund) -> Self {
        Self {
            id: value.id,
            deposit_id: value.deposit_id,
            amount: value.amount,
            reason: value.reason,
            refunded_at: value.refunded_at,
            created_at: value.created_at,
        }
    }
}
