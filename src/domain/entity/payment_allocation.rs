use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentAllocation {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub allocated_at: DateTime<Utc>,
    pub reversed_at: Option<DateTime<Utc>>,
}

impl PaymentAllocation {
    pub fn reverse(&mut self, reversed_at: DateTime<Utc>) -> Result<(), String> {
        if self.reversed_at.is_some() {
            return Err("payment allocation already reversed".into());
        }

        self.reversed_at = Some(reversed_at);

        Ok(())
    }
}
