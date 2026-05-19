use chrono::NaiveDate;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct HousekeepingRoomDailyStateInput {
    pub room_id: Uuid,
    pub service_date: NaiveDate,
}
