use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum CompanyStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub status: CompanyStatus,
}

impl Company {
    pub fn new(
        id: Uuid,
        name: String,
    ) -> Result<Self, String> {

        if name.trim().is_empty() {
            return Err("company name cannot be empty".into());
        }

        Ok(Self {
            id,
            name,
            status: CompanyStatus::Active,
        })
    }

    pub fn deactivate(&mut self) {
        self.status = CompanyStatus::Inactive;
    }
}