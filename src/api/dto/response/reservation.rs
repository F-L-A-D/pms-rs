use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDate, Utc};

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
    pub version: i64,
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
    pub department_code: Option<String>,
    pub account_code: Option<String>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationEditSessionResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub actor_id: String,
    pub actor_label: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationEditSessionWarningResponse {
    pub active_sessions: Vec<ReservationEditSessionResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenReservationEditSessionResponse {
    pub session: ReservationEditSessionResponse,
    pub warning: Option<ReservationEditSessionWarningResponse>,
}
