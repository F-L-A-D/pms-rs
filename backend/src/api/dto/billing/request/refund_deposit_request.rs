use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefundDepositRequest {
    pub amount: String,
    pub reason: Option<String>,
}
