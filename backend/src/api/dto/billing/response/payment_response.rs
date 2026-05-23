use chrono::{
    DateTime,
    Utc,
};

use rust_decimal::Decimal;

use serde::{
    Deserialize,
    Serialize,
};

use uuid::Uuid;

use crate::domain::entity::payment::{
    Payment,
    PaymentMethod,
    PaymentStatus,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub unapplied_amount: Decimal,
    pub refunded_amount: Decimal,
    pub status: PaymentStatus,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub paid_at: DateTime<Utc>,
}

impl From<Payment> for PaymentResponse {
    fn from(payment: Payment) -> Self {
        Self {
            id: payment.id,
            folio_id: payment.folio_id,
            amount: payment.amount,
            unapplied_amount: payment.unapplied_amount,
            refunded_amount: payment.refunded_amount,
            status: payment.status,
            method: payment.method,
            external_reference: payment.external_reference,
            paid_at: payment.paid_at,
        }
    }
}