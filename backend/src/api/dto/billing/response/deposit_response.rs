use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::{
    deposit::{Deposit, DepositStatus},
    payment::PaymentMethod,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub unapplied_amount: Decimal,
    pub refunded_amount: Decimal,
    pub status: DepositStatus,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub received_at: DateTime<Utc>,
}

impl From<Deposit> for DepositResponse {
    fn from(deposit: Deposit) -> Self {
        Self {
            id: deposit.id,
            folio_id: deposit.folio_id,
            amount: deposit.amount,
            unapplied_amount: deposit.unapplied_amount,
            refunded_amount: deposit.refunded_amount,
            status: deposit.status,
            method: deposit.method,
            external_reference: deposit.external_reference,
            received_at: deposit.received_at,
        }
    }
}
