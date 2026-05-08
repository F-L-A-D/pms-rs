use chrono::{
    NaiveDate,
    DateTime,
    Utc,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GuestSummaryProjection {
    pub guest_id: Uuid,
    pub total_stays: i64,
    pub total_nights: i64,
    pub total_spending: i64,
    pub last_stay_at: Option<NaiveDate>,    
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}