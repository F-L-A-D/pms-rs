use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    domain::entity::guest::{Gender, Guest},
    domain::semantic::guest_preference::{GuestPreference, GuestPreferenceType},
};

#[derive(Debug, Deserialize, Serialize)]
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
    fn from(guest: Guest) -> Self {
        Self {
            id: guest.id,

            last_name: guest.profile.last_name,

            first_name: guest.profile.first_name,

            phone: guest.profile.phone,

            email: guest.profile.email,

            nationality: guest.profile.nationality,

            birth_date: guest.profile.birth_date,

            gender: guest.profile.gender,

            membership_code: guest.profile.membership_code,

            marketing_opt_in: guest.profile.marketing_opt_in,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuestPreferenceResponse {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub preference_type: GuestPreferenceType,
    pub value: String,
    pub notes: Option<String>,
}

impl From<GuestPreference> for GuestPreferenceResponse {
    fn from(preference: GuestPreference) -> Self {
        Self {
            id: preference.id,
            guest_id: preference.guest_id,
            preference_type: preference.preference_type,
            value: preference.value,
            notes: preference.notes,
        }
    }
}
