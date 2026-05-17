use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::semantic::reservation_guest_relation::ReservationGuestRelationType;

pub struct CreateReservationInput {
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub participants: Vec<ReservationParticipantInput>,
}

pub struct ReservationParticipantInput {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

pub struct ModifyReservationInput {
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub room_class: Option<String>,
}
