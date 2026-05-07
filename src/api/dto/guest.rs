use serde::{
    Serialize,
    Deserialize,
};

use chrono::NaiveDate;

use crate::domain::guest::{
    Guest,
    Gender,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuestRequest {
    pub id: String,
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

#[derive(Debug, Serialize)]
pub struct GuestResponse {
    pub id: String,
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

    fn from(guest: Guest) -> Self {

        Self {
            id: guest.id,
            last_name: guest.last_name,
            first_name: guest.first_name,
            phone: guest.phone,
            email: guest.email,
            nationality: guest.nationality,
            birth_date: guest.birth_date,
            gender: guest.gender,
            membership_code: guest.membership_code,
            marketing_opt_in: guest.marketing_opt_in,
        }
    }
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
    pub name: Option<String>,
}