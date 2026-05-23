use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct DepositRefund {
    pub id: Uuid,
    pub deposit_id: Uuid,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub refunded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl DepositRefund {
    pub fn new(
        id: Uuid,
        deposit_id: Uuid,
        amount: Decimal,
        reason: Option<String>,
        refunded_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if amount <= Decimal::ZERO {
            return Err("deposit refund amount must be positive".into());
        }

        Ok(Self {
            id,
            deposit_id,
            amount,
            reason,
            refunded_at,
            created_at,
        })
    }
}
