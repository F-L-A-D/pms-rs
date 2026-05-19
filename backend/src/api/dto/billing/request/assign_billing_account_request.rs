use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignBillingAccountRequest {
    pub folio_id: String,
    pub billing_account_id: String,
}
