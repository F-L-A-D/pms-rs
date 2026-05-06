use chrono::{
    DateTime,
    Utc
};

#[derive(Debug, Clone, PartialEq)]
pub struct Guest {
    pub id: String,
    pub last_name: String,
    pub first_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Guest {

    pub fn new(
        id: String,
        last_name: String,
        first_name: String,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<Self, String> {

        if last_name.trim().is_empty() {
            return Err("last_name required".into());
        }

        if first_name.trim().is_empty() {
            return Err("first_name required".into());
        }
        
        let now = Utc::now();

        Ok(
            Self {
                id,
                last_name: last_name.trim().to_string(),
                first_name: first_name.trim().to_string(),
                phone,
                email: email.map(|v| v.trim().to_lowercase()),
                created_at: now,
                updated_at: now,
            }
        )
    }

    pub fn update_profile(
        &mut self,
        last_name: String,
        first_name: String,
        phone: Option<String>,
        email: Option<String>,
    ) -> Result<(), String> {

        if last_name.trim().is_empty() {
            return Err("last_name required".into());
        }

        if first_name.trim().is_empty() {
            return Err("first_name required".into());
        }

        self.last_name = last_name.trim().to_string();
        self.first_name = first_name.trim().to_string();
        self.phone = phone;
        self.email =
            email.map(|v| v.trim().to_lowercase());

        self.updated_at = Utc::now();

        Ok(())
    }
    
}