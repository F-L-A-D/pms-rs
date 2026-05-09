use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};

use uuid::Uuid;

use crate::domain::reservation::{
    ReservationStatus,
    StayStatus,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationSearchProjection {
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
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}