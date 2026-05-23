use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReverseDepositApplicationRequest {
    pub reason: Option<String>,
}
