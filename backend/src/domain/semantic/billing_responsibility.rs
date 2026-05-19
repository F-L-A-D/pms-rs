use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BillingResponsibility {
    Guest(Uuid),
    Company(Uuid),
    Agent(Uuid),
}
