use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use crate::domain::semantic::reservation_booking::ReservationRevenueCategory;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevenueSummaryLine {
    pub service_date: Option<NaiveDate>,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: Decimal,
}
