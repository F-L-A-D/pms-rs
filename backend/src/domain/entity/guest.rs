use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
    Unspecified,
}

impl Gender {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
            Self::Other => "other",
            Self::Unspecified => "unspecified",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "male" => Some(Self::Male),
            "female" => Some(Self::Female),
            "other" => Some(Self::Other),
            "unspecified" => Some(Self::Unspecified),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GuestSearchField {
    FirstName,
    LastName,
    Email,
    Phone,
    MembershipCode,
    Nationality,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuestProfile {
    pub last_name: String,
    pub first_name: String,

    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,

    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,

    pub membership_code: Option<String>,

    pub marketing_opt_in: bool,
}

impl GuestProfile {
    pub fn new(
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
        let profile = Self {
            last_name: last_name.trim().to_string(),

            first_name: first_name.trim().to_string(),

            phone: Self::normalize_optional_string(phone),

            email: Self::normalize_email(email)?,

            nationality: Self::normalize_optional_string(nationality),

            birth_date,

            gender,

            membership_code: Self::normalize_optional_string(membership_code),

            marketing_opt_in,
        };

        profile.validate()?;

        Ok(profile)
    }

    fn validate(&self) -> Result<(), String> {
        if self.last_name.trim().is_empty() {
            return Err("last_name required".into());
        }

        if self.first_name.trim().is_empty() {
            return Err("first_name required".into());
        }

        if let Some(email) = &self.email {
            if !email.contains('@') {
                return Err("invalid email".into());
            }
        }

        Ok(())
    }

    fn normalize_email(email: Option<String>) -> Result<Option<String>, String> {
        let normalized = Self::normalize_optional_string(email).map(|v| v.to_lowercase());

        Ok(normalized)
    }

    fn normalize_optional_string(value: Option<String>) -> Option<String> {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Guest {
    pub id: Uuid,

    pub profile: GuestProfile,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Guest {
    pub fn new(id: Uuid, profile: GuestProfile) -> Self {
        let now = Utc::now();

        Self {
            id,

            profile,

            created_at: now,

            updated_at: now,
        }
    }

    pub fn replace_profile(&mut self, profile: GuestProfile) {
        self.profile = profile;

        self.updated_at = Utc::now();
    }
}
