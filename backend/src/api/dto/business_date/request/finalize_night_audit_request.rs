use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FinalizeNightAuditRequest {
    pub reason: Option<String>,
}
