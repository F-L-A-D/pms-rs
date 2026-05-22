use serde::{Deserialize, Serialize};

use crate::domain::entity::payment::PaymentMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateReceivablePaymentRequest {
    pub amount: String,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
    pub reason: Option<String>,
}
