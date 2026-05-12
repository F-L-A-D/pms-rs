use serde::{
    Deserialize, 
    Serialize,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::{
    reservation_guest_relation
        ::ReservationGuestRelationType,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationParticipantInput {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    #[serde(alias = "id")]
    pub external_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub participants: Vec<ReservationParticipantInput>,
}

#[derive(Debug, Deserialize)]
pub struct ModifyReservationRequest {
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub room_class: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReservationParticipantResponse {
    pub guest_id: Uuid,
    pub relation_type: String,
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: String,
    pub stay_status: Option<String>,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub participants: Vec<ReservationParticipantResponse>,
}

#[derive(Debug, Serialize)]
pub struct ReservationSearchResponse {
    pub reservation_id: Uuid,
    pub external_id: Option<String>,
    pub primary_guest_name: String,
    pub participant_names: Vec<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub reservation_status: String,
    pub stay_status: Option<String>,
}