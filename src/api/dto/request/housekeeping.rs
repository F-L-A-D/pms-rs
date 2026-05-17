use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HousekeepingRoomDailyStateRequest {
    pub service_date: String,
}
