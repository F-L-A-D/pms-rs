use chrono::NaiveDate;

use uuid::Uuid;

use rust_decimal::Decimal;

use crate::domain::{
    entity::guest::Gender,
    semantic::{
        reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
        reservation_guest_relation::ReservationGuestRelationType,
        reservation_note::ReservationNoteKind,
    },
};

pub struct CreateReservationInput {
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_class: String,
    pub booking_channel: ReservationBookingChannel,
    pub plan_code: Option<String>,
    pub package_breakdowns: Vec<ReservationPackageBreakdownInput>,
    pub daily_details: Vec<ReservationDailyDetailInput>,
    pub participants: Vec<ReservationParticipantInput>,
}

pub struct ReservationPackageBreakdownInput {
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: Decimal,
}

pub struct ReservationDailyDetailInput {
    pub service_date: NaiveDate,
    pub room_class: String,
    pub plan_code: Option<String>,
    pub adult_count: i64,
    pub child_count: i64,
    pub sleep_sharing_child_count: i64,
    pub sleep_sharing_children: Vec<ReservationSleepSharingChildInput>,
    pub package_breakdowns: Vec<ReservationPackageBreakdownInput>,
}

pub struct ReservationSleepSharingChildInput {
    pub name: Option<String>,
    pub age: Option<i64>,
    pub gender: Option<Gender>,
}

pub struct ReservationDailyRevenueAllocationInput {
    pub service_date: NaiveDate,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: Decimal,
}

pub struct ReservationParticipantInput {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

pub struct OpenReservationEditSessionInput {
    pub reservation_id: Uuid,
    pub actor_id: String,
    pub actor_label: Option<String>,
    pub lease_minutes: Option<i64>,
}

pub struct CloseReservationEditSessionInput {
    pub session_id: Uuid,
    pub actor_id: String,
}

pub struct ModifyReservationInput {
    pub expected_version: Option<i64>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub room_class: Option<String>,
    pub package_breakdowns: Option<Vec<ReservationPackageBreakdownInput>>,
    pub daily_details: Option<Vec<ReservationDailyDetailInput>>,
    pub daily_revenue_allocations: Option<Vec<ReservationDailyRevenueAllocationInput>>,
    pub participants: Option<Vec<ReservationParticipantInput>>,
}

pub struct CreateReservationNoteInput {
    pub reservation_id: Uuid,
    pub kind: ReservationNoteKind,
    pub department_code: Option<String>,
    pub body: String,
    pub actor_id: Option<String>,
}

pub struct CreateReservationTraceInput {
    pub reservation_id: Uuid,
    pub department_code: String,
    pub body: String,
    pub actor_id: Option<String>,
}

pub struct ResolveReservationTraceInput {
    pub reservation_id: Uuid,
    pub trace_id: Uuid,
    pub actor_id: Option<String>,
}

pub struct DeleteReservationNoteInput {
    pub reservation_id: Uuid,
    pub note_id: Uuid,
    pub actor_id: Option<String>,
}
