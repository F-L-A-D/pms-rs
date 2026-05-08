use chrono::{
    NaiveDate,
    DateTime,
    Utc
};

use serde::{
    Serialize,
    Deserialize,
};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Guest {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub membership_code: Option<String>,
    pub marketing_opt_in: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Guest {

    pub fn new(
        id: Uuid,
        last_name: String,
        first_name: String,
        phone: Option<String>,
        email: Option<String>,
        nationality: Option<String>,
        birth_date: Option<NaiveDate>,
        gender: Option<Gender>,
        membership_code: Option<String>,
        marketing_opt_in: bool,
    ) -> Result<Self, String> {

        let last_name = last_name.trim().to_string();
        let first_name = first_name.trim().to_string();

        if last_name.is_empty() {
            return Err("last_name required".into());
        }

        if first_name.is_empty() {
            return Err("first_name required".into());
        }

        let email =
            Self::normalize_optional_string(email)
                .map(|v| v.to_lowercase());

        if let Some(email) = &email {
            if !email.contains('@') {
                return Err("invalid email".into());
            }
        }

        let now = Utc::now();

        Ok(
            Self {
                id,
                last_name,
                first_name,
                phone: Self::normalize_optional_string(phone),
                email,
                nationality:
                    Self::normalize_optional_string(nationality),
                birth_date,
                gender,
                membership_code:
                    Self::normalize_optional_string(
                        membership_code
                    ),
                marketing_opt_in,
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
        nationality: Option<String>,
        birth_date: Option<NaiveDate>,
        gender: Option<Gender>,
        membership_code: Option<String>,
        marketing_opt_in: bool,
    ) -> Result<(), String> {

        let last_name = last_name.trim().to_string();
        let first_name = first_name.trim().to_string();

        if last_name.is_empty() {
            return Err("last_name required".into());
        }

        if first_name.is_empty() {
            return Err("first_name required".into());
        }

        let email =
            Self::normalize_optional_string(email)
                .map(|v| v.to_lowercase());

        if let Some(email) = &email {
            if !email.contains('@') {
                return Err("invalid email".into());
            }
        }

        self.last_name = last_name;
        self.first_name = first_name;
        self.phone =
            Self::normalize_optional_string(phone);
        self.email = email;
        self.nationality =
            Self::normalize_optional_string(nationality);
        self.birth_date = birth_date;
        self.gender = gender;
        self.membership_code =
            Self::normalize_optional_string(
                membership_code
            );
        self.marketing_opt_in = marketing_opt_in;

        self.updated_at = Utc::now();

        Ok(())
    }

    fn normalize_optional_string(
        value: Option<String>
    ) -> Option<String> {

        value.and_then(|v| {
            let trimmed = v.trim().to_string();

            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
    }
}