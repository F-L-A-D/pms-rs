use chrono::NaiveDate;

use serde::Serialize;

#[derive(Serialize)]
pub struct GuestSummaryResponse {
    pub total_stays: i64,
    pub total_nights: i64,
    pub total_spending: i64,
    pub last_stay_at: Option<NaiveDate>,
}