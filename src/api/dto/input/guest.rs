use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::entity::guest::{Gender, GuestSearchField};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuestInput {
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

#[derive(Debug)]
pub struct UpdateGuestInput {
    pub guest_id: Uuid,

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

#[derive(Debug)]
pub struct GuestSearchInput {
    pub query: Option<String>,
    pub field: Option<GuestSearchField>,
}
