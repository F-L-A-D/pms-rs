use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct StartNightAuditRequest {
    pub reason: Option<String>,
}
