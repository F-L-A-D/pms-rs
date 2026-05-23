use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReversePaymentAllocationRequest {
    pub reason: Option<String>,
}
