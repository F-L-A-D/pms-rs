use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::reservation::{
    ReservationStatus,
    StayStatus,
};

#[derive(Debug, Clone)]
pub struct ReservationSearchView {
    pub reservation_id: Uuid,
    pub external_id: String,
    pub primary_guest_name: String,
    pub participant_names: Vec<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub room_id: Option<String>,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
}