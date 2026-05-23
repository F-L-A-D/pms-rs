use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignBillingAccountRequest {
    pub billing_account_id: String,
}
