use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};

use serde::{
    Deserialize,
    Serialize,
};

use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub enum Gender {
    Male,
    Female,
    Other,
    Unspecified,
}

#[derive(Debug, Clone)]
pub struct GuestProfileUpdate {
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
        profile: GuestProfileUpdate,
    ) -> Result<Self, String> {

        Self::validate_profile(
            &profile
        )?;

        let now =
            Utc::now();

        Ok(
            Self {

                id,

                last_name:
                    profile.last_name
                        .trim()
                        .to_string(),

                first_name:
                    profile.first_name
                        .trim()
                        .to_string(),

                phone:
                    Self::normalize_optional_string(
                        profile.phone
                    ),

                email:
                    Self::normalize_email(
                        profile.email
                    )?,

                nationality:
                    Self::normalize_optional_string(
                        profile.nationality
                    ),

                birth_date:
                    profile.birth_date,

                gender:
                    profile.gender,

                membership_code:
                    Self::normalize_optional_string(
                        profile.membership_code
                    ),

                marketing_opt_in:
                    profile.marketing_opt_in,

                created_at:
                    now,

                updated_at:
                    now,
            }
        )
    }

    pub fn update_profile(
        &mut self,
        update: GuestProfileUpdate,
    ) -> Result<(), String> {

        Self::validate_profile(
            &update
        )?;

        self.last_name =
            update.last_name
                .trim()
                .to_string();

        self.first_name =
            update.first_name
                .trim()
                .to_string();

        self.phone =
            Self::normalize_optional_string(
                update.phone
            );

        self.email =
            Self::normalize_email(
                update.email
            )?;

        self.nationality =
            Self::normalize_optional_string(
                update.nationality
            );

        self.birth_date =
            update.birth_date;

        self.gender =
            update.gender;

        self.membership_code =
            Self::normalize_optional_string(
                update.membership_code
            );

        self.marketing_opt_in =
            update.marketing_opt_in;

        self.updated_at =
            Utc::now();

        Ok(())
    }

    fn validate_profile(
        profile: &GuestProfileUpdate,
    ) -> Result<(), String> {

        if profile.last_name
            .trim()
            .is_empty()
        {

            return Err(
                "last_name required"
                    .into()
            );
        }

        if profile.first_name
            .trim()
            .is_empty()
        {

            return Err(
                "first_name required"
                    .into()
            );
        }

        if let Some(email)
            = &profile.email
        {

            if !email.contains('@')
            {

                return Err(
                    "invalid email"
                        .into()
                );
            }
        }

        Ok(())
    }

    fn normalize_email(
        email: Option<String>,
    ) -> Result<Option<String>, String> {

        let normalized =
            Self::normalize_optional_string(
                email
            )
            .map(
                |v| v.to_lowercase()
            );

        Ok(normalized)
    }

    fn normalize_optional_string(
        value: Option<String>,
    ) -> Option<String> {

        value.and_then(|v| {

            let trimmed =
                v.trim()
                    .to_string();

            if trimmed.is_empty() {

                None

            } else {

                Some(trimmed)
            }
        })
    }
}