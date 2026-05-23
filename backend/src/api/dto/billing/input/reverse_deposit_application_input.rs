use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReverseDepositApplicationInput {
    pub deposit_application_id: Uuid,
    pub reason: Option<String>,
}
