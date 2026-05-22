use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateBillingAccountInput {
    pub company_id: Option<Uuid>,
    pub name: String,
}