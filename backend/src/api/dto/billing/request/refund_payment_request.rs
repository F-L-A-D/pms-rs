use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefundPaymentRequest {
    pub amount: String,
    pub reason: Option<String>,
}
