use chrono::NaiveDate;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::semantic::reservation_guest_relation::ReservationGuestRelationType;

pub use crate::api::dto::response::reservation::ReservationResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    #[serde(alias = "id")]
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub participants: Vec<ReservationParticipantInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationParticipantInput {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}
