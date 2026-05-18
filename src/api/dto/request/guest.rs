use serde::{Deserialize, Serialize};

use crate::domain::entity::guest::{Gender, GuestSearchField};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuestRequest {
    pub last_name: String,
    pub first_name: String,

    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,

    pub birth_date: Option<String>,
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

    pub birth_date: Option<String>,

    pub gender: Option<Gender>,

    pub membership_code: Option<String>,

    #[serde(default)]
    pub marketing_opt_in: bool,
}

#[derive(Debug, Deserialize)]
pub struct GuestSearchQuery {
    pub query: Option<String>,
    pub field: Option<GuestSearchField>,
}
