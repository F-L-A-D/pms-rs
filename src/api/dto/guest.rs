use serde::{
    Deserialize,
    Serialize,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::guest::{
    Gender,
    Guest,
    GuestProfileUpdate,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuestRequest {
    pub last_name: String,
    pub first_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub membership_code: Option<String>,

    #[serde(default)]
    pub marketing_opt_in: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGuestRequest {
    pub last_name: String,
    pub first_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub membership_code: Option<String>,

    #[serde(default)]
    pub marketing_opt_in: bool,
}

#[derive(Debug, Deserialize)]
pub struct GuestSearchQuery {
    pub query: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GuestResponse {
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
}

impl From<Guest> for GuestResponse {

    fn from(
        guest: Guest,
    ) -> Self {

        Self {
            id:
                guest.id,

            last_name:
                guest.last_name,

            first_name:
                guest.first_name,

            phone:
                guest.phone,

            email:
                guest.email,

            nationality:
                guest.nationality,

            birth_date:
                guest.birth_date,

            gender:
                guest.gender,

            membership_code:
                guest.membership_code,

            marketing_opt_in:
                guest.marketing_opt_in,
        }
    }
}

impl UpdateGuestRequest {

    pub fn into_profile_update(
        self,
    ) -> GuestProfileUpdate {

        GuestProfileUpdate {
            last_name:
                self.last_name,

            first_name:
                self.first_name,

            phone:
                self.phone,

            email:
                self.email,

            nationality:
                self.nationality,

            birth_date:
                self.birth_date,

            gender:
                self.gender,

            membership_code:
                self.membership_code,

            marketing_opt_in:
                self.marketing_opt_in,
        }
    }
}