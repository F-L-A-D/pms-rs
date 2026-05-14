use chrono::{
    NaiveDate,
    DateTime,
    Utc,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GuestAggregate {
    pub guest_id: Uuid,
    pub total_stays: i64,
    pub total_nights: i64,
    pub total_spending: i64,
    pub last_stay_at: Option<NaiveDate>,    
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for GuestAggregate {

    fn eq(
        &self,
        other: &Self,
    ) -> bool {

        self.guest_id
            == other.guest_id

        &&
        
        self.total_stays
            == other.total_stays

        &&

        self.total_nights
            == other.total_nights

        &&

        self.total_spending
            == other.total_spending

        &&

        self.last_stay_at
            == other.last_stay_at

        &&

        self.projection_version
            == other.projection_version
    }
}