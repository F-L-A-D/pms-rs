use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::{
    entity::reservation::{ReservationStatus, StayStatus},
    semantic::{
        reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
        reservation_guest_relation::ReservationGuestRelationType,
    },
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
    pub booking_channel: ReservationBookingChannel,
    pub plan_code: Option<String>,
    pub package_breakdowns: Vec<ReservationPackageBreakdownResponse>,
    pub daily_details: Vec<ReservationDailyDetailResponse>,
    pub daily_revenue_allocations: Vec<ReservationDailyRevenueAllocationResponse>,
    pub participants: Vec<ReservationParticipantResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationPackageBreakdownResponse {
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDailyDetailResponse {
    pub service_date: NaiveDate,
    pub room_class: String,
    pub plan_code: Option<String>,
    pub adult_count: i64,
    pub child_count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDailyRevenueAllocationResponse {
    pub service_date: NaiveDate,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: rust_decimal::Decimal,
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
    pub booking_channel: ReservationBookingChannel,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
}
