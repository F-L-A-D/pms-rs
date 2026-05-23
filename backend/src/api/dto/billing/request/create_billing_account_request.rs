use serde::Deserialize;
use uuid::Uuid;

use crate::api::dto::billing::input::create_billing_account_input::CreateBillingAccountInput;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBillingAccountRequest {
    pub company_id: Option<Uuid>,
    pub name: String,
}

impl From<CreateBillingAccountRequest> for CreateBillingAccountInput {
    fn from(request: CreateBillingAccountRequest) -> Self {
        Self {
            company_id: request.company_id,
            name: request.name,
        }
    }
}
