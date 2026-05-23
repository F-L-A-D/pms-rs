use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::Serialize;

use uuid::Uuid;

use crate::domain::entity::payment_refund::PaymentRefund;

#[derive(Debug, Serialize)]
pub struct PaymentRefundResponse {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refunded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<PaymentRefund> for PaymentRefundResponse {
    fn from(value: PaymentRefund) -> Self {
        Self {
            id: value.id,
            payment_id: value.payment_id,
            amount: value.amount,
            reason: value.reason,
            refunded_at: value.refunded_at,
            created_at: value.created_at,
        }
    }
}
