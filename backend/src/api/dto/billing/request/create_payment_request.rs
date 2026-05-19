use serde::{Deserialize, Serialize};

use crate::domain::entity::payment::PaymentMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub folio_id: String,
    pub amount: String,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
}
