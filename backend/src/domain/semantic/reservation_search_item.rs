use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};

use uuid::Uuid;

use crate::domain::{
    entity::reservation::{
        ReservationStatus,
        StayStatus,
    },
    semantic::reservation_linked_resources::ReservationLinkedResources,
};

#[derive(Debug, Clone)]
pub struct ReservationSearchItem {
    pub id: Uuid,
    pub external_id: Option<String>,

    pub check_in: NaiveDate,
    pub check_out: NaiveDate,

    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,

    pub room_class: Option<String>,
    pub room_id: Option<Uuid>,

    pub primary_guest_name: Option<String>,
    pub linked_resources: ReservationLinkedResources,

    pub created_at: DateTime<Utc>,
}