use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use crate::domain::semantic::{
    reservation_booking::ReservationRevenueCategory, revenue_summary::RevenueSummaryLine,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSummaryLineResponse {
    pub service_date: Option<NaiveDate>,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: Decimal,
}

impl From<RevenueSummaryLine> for RevenueSummaryLineResponse {
    fn from(line: RevenueSummaryLine) -> Self {
        Self {
            service_date: line.service_date,
            revenue_category: line.revenue_category,
            department_code: line.department_code,
            account_code: line.account_code,
            amount: line.amount,
        }
    }
}
