use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateReceivableStatusRequest {
    pub reason: Option<String>,
}