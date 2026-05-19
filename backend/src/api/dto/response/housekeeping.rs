use chrono::{DateTime, NaiveDate, Utc};

use serde::Serialize;

use uuid::Uuid;

use crate::domain::semantic::room_daily_state::{
    RoomDailyHousekeepingStatus, RoomDailyOccupancyStatus, RoomDailyState,
};

#[derive(Debug, Serialize)]
pub struct RoomDailyStateResponse {
    pub room_id: Uuid,
    pub service_date: NaiveDate,
    pub occupancy_status: RoomDailyOccupancyStatus,
    pub housekeeping_status: RoomDailyHousekeepingStatus,
    pub updated_at: DateTime<Utc>,
}

impl From<RoomDailyState> for RoomDailyStateResponse {
    fn from(state: RoomDailyState) -> Self {
        Self {
            room_id: state.room_id,
            service_date: state.service_date,
            occupancy_status: state.occupancy_status,
            housekeeping_status: state.housekeeping_status,
            updated_at: state.updated_at,
        }
    }
}
