use serde::{Deserialize, Serialize};

use crate::domain::{
    entity::guest::Gender,
    semantic::{
        reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
        reservation_guest_relation::ReservationGuestRelationType,
        reservation_note::ReservationNoteKind,
    },
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
    #[serde(default)]
    pub daily_details: Vec<ReservationDailyDetailRequest>,
    pub participants: Vec<ReservationParticipantRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationPackageBreakdownRequest {
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationDailyDetailRequest {
    pub service_date: String,
    pub room_class: String,
    pub plan_code: Option<String>,
    #[serde(default = "default_adult_count")]
    pub adult_count: i64,
    #[serde(default)]
    pub child_count: i64,
    #[serde(default)]
    pub sleep_sharing_child_count: i64,
    #[serde(default)]
    pub sleep_sharing_children: Vec<ReservationSleepSharingChildRequest>,
    #[serde(default)]
    pub package_breakdowns: Vec<ReservationPackageBreakdownRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationSleepSharingChildRequest {
    pub name: Option<String>,
    pub age: Option<i64>,
    pub gender: Option<Gender>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationDailyRevenueAllocationRequest {
    pub service_date: String,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: String,
}

fn default_adult_count() -> i64 {
    1
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReservationParticipantRequest {
    pub guest_id: String,
    pub relation_type: ReservationGuestRelationType,
}

#[derive(Debug, Deserialize)]
pub struct OpenReservationEditSessionRequest {
    pub actor_id: String,
    pub actor_label: Option<String>,
    pub lease_minutes: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CloseReservationEditSessionRequest {
    pub actor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifyReservationRequest {
    pub expected_version: Option<i64>,
    pub check_in: Option<String>,
    pub check_out: Option<String>,
    pub room_class: Option<String>,
    pub package_breakdowns: Option<Vec<ReservationPackageBreakdownRequest>>,
    pub daily_details: Option<Vec<ReservationDailyDetailRequest>>,
    pub daily_revenue_allocations: Option<Vec<ReservationDailyRevenueAllocationRequest>>,
    pub participants: Option<Vec<ReservationParticipantRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReservationNoteRequest {
    pub kind: ReservationNoteKind,
    pub department_code: Option<String>,
    pub body: String,
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReservationTraceRequest {
    pub department_code: String,
    pub body: String,
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResolveReservationTraceRequest {
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteReservationNoteRequest {
    pub actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteReservationTraceRequest {
    pub actor_id: Option<String>,
}
