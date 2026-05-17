use serde::{Deserialize, Serialize};

use crate::domain::semantic::{
    reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
    reservation_guest_relation::ReservationGuestRelationType,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReservationRequest {
    #[serde(alias = "id")]
    pub external_id: Option<String>,
    pub check_in: String,
    pub check_out: String,
    pub room_class: String,
    pub booking_channel: Option<ReservationBookingChannel>,
    pub plan_code: Option<String>,
    #[serde(default)]
    pub package_breakdowns: Vec<ReservationPackageBreakdownRequest>,
    pub participants: Vec<ReservationParticipantRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationPackageBreakdownRequest {
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationParticipantRequest {
    pub guest_id: String,
    pub relation_type: ReservationGuestRelationType,
}

#[derive(Debug, Deserialize)]
pub struct ModifyReservationRequest {
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub room_class: Option<String>,
}
