use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct DepositApplication {
    pub id: Uuid,
    pub deposit_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub applied_at: DateTime<Utc>,
    pub reversed_at: Option<DateTime<Utc>>,
}

impl DepositApplication {
    pub fn new(
        id: Uuid,
        deposit_id: Uuid,
        receivable_id: Uuid,
        amount: Decimal,
        applied_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if amount <= Decimal::ZERO {
            return Err("deposit application amount must be positive".into());
        }

        Ok(Self {
            id,
            deposit_id,
            receivable_id,
            amount,
            applied_at,
            reversed_at: None,
        })
    }

    pub fn reverse(&mut self, reversed_at: DateTime<Utc>) -> Result<(), String> {
        if self.reversed_at.is_some() {
            return Err("deposit application is already reversed".into());
        }

        self.reversed_at = Some(reversed_at);

        Ok(())
    }
}