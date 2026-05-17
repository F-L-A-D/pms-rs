use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::entity::payment::PaymentMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub paid_at: DateTime<Utc>,
}
