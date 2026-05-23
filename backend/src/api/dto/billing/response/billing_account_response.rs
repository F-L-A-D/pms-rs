use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entity::billing_account::{BillingAccount, BillingAccountStatus};

#[derive(Debug, Clone, Serialize)]
pub struct BillingAccountResponse {
    pub id: Uuid,
    pub company_id: Option<Uuid>,
    pub name: String,
    pub status: BillingAccountStatus,
    pub created_at: DateTime<Utc>,
}

impl From<BillingAccount> for BillingAccountResponse {
    fn from(account: BillingAccount) -> Self {
        Self {
            id: account.id,
            company_id: account.company_id,
            name: account.name,
            status: account.status,
            created_at: account.created_at,
        }
    }
}
