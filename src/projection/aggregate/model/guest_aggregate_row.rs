use chrono::NaiveDate;

#[derive(Debug)]
pub struct GuestAggregateRow {
    pub total_stays: i64,
    pub total_nights: i64,
    pub total_spending: i64,
    pub last_stay_at: Option<NaiveDate>,
}
