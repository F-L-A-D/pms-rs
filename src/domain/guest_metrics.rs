use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct GuestMetrics {
    pub total_stays: i64,
    pub total_nights: i64,
    pub total_spending: i64,
    pub last_stay_at: Option<NaiveDate>,
}