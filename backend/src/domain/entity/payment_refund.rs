use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentRefund {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refunded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl PaymentRefund {
    pub fn new(
        id: Uuid,
        payment_id: Uuid,
        amount: Decimal,
        reason: Option<String>,
        refunded_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if amount <= Decimal::ZERO {
            return Err("refund amount must be positive".into());
        }

        Ok(Self {
            id,
            payment_id,
            amount,
            reason,
            refunded_at,
            created_at,
        })
    }
}
