use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateExistingPaymentRequest {
    pub receivable_id: String,
    pub amount: String,
    pub reason: Option<String>,
}
