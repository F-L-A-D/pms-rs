use chrono::NaiveDate;

use uuid::Uuid;

use rust_decimal::Decimal;

use crate::domain::semantic::{
    reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
    reservation_guest_relation::ReservationGuestRelationType,
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
    pub package_breakdowns: Vec<ReservationPackageBreakdownInput>,
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

pub struct ModifyReservationInput {
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub room_class: Option<String>,
    pub package_breakdowns: Option<Vec<ReservationPackageBreakdownInput>>,
    pub daily_details: Option<Vec<ReservationDailyDetailInput>>,
    pub daily_revenue_allocations: Option<Vec<ReservationDailyRevenueAllocationInput>>,
    pub participants: Option<Vec<ReservationParticipantInput>>,
}
