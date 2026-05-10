use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum BillingAccountStatus {
    Active,
    Suspended,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BillingAccount {
    pub id: Uuid,
    pub company_id: Option<Uuid>,
    pub name: String,
    pub status: BillingAccountStatus,
}

impl BillingAccount {

    pub fn new(
        id: Uuid,
        company_id: Option<Uuid>,
        name: String,
    ) -> Result<Self, String> {

        if name.trim().is_empty() {
            return Err(
                "billing account name cannot be empty".into()
            );
        }

        Ok(Self {
            id,
            company_id,
            name,
            status: BillingAccountStatus::Active,
        })
    }

    pub fn suspend(&mut self) {
        self.status =
            BillingAccountStatus::Suspended;
    }
}