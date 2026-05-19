use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AssignBillingAccountInput {
    pub folio_id: Uuid,
    pub billing_account_id: Uuid,
}
