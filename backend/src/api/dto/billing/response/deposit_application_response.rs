use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::Serialize;

use uuid::Uuid;

use crate::domain::entity::deposit_application::DepositApplication;

#[derive(Debug, Clone, Serialize)]
pub struct DepositApplicationResponse {
    pub id: Uuid,
    pub deposit_id: Uuid,
    pub receivable_id: Uuid,
    pub amount: Decimal,
    pub applied_at: DateTime<Utc>,
    pub reversed_at: Option<DateTime<Utc>>,
}

impl From<DepositApplication> for DepositApplicationResponse {
    fn from(application: DepositApplication) -> Self {
        Self {
            id: application.id,
            deposit_id: application.deposit_id,
            receivable_id: application.receivable_id,
            amount: application.amount,
            applied_at: application.applied_at,
            reversed_at: application.reversed_at,
        }
    }
}
