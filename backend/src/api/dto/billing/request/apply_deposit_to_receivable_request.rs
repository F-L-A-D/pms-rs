use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApplyDepositToReceivableRequest {
    pub receivable_id: String,
    pub amount: String,
    pub reason: Option<String>,
}