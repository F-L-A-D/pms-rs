use crate::api::dto::business_date::response::business_date_response::BusinessDateResponse;

#[derive(Clone, Debug, serde::Serialize)]
pub struct StartNightAuditResponse {
    pub business_date: BusinessDateResponse,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FinalizeNightAuditResponse {
    pub closed_business_date: BusinessDateResponse,
    pub current_business_date: BusinessDateResponse,
}
