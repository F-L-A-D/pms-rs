use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::{
    entity::reservation::{ReservationStatus, StayStatus},
    semantic::reservation_guest_relation::ReservationGuestRelationType,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub participants: Vec<ReservationParticipantResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationParticipantResponse {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationSearchResponse {
    pub reservation_id: Uuid,
    pub external_id: Option<String>,
    pub primary_guest_name: String,
    pub participant_names: Vec<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
}
