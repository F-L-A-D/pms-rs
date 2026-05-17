use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomDailyOccupancyStatus {
    Vacant,
    Occupied,
    OutOfOrder,
}

impl RoomDailyOccupancyStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Vacant => "vacant",
            Self::Occupied => "occupied",
            Self::OutOfOrder => "out_of_order",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "vacant" => Some(Self::Vacant),
            "occupied" => Some(Self::Occupied),
            "out_of_order" => Some(Self::OutOfOrder),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomDailyHousekeepingStatus {
    Dirty,
    Cleaning,
    Cleaned,
    Inspected,
}

impl RoomDailyHousekeepingStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Dirty => "dirty",
            Self::Cleaning => "cleaning",
            Self::Cleaned => "cleaned",
            Self::Inspected => "inspected",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "dirty" => Some(Self::Dirty),
            "cleaning" => Some(Self::Cleaning),
            "cleaned" => Some(Self::Cleaned),
            "inspected" => Some(Self::Inspected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomDailyState {
    pub room_id: Uuid,
    pub service_date: NaiveDate,
    pub occupancy_status: RoomDailyOccupancyStatus,
    pub housekeeping_status: RoomDailyHousekeepingStatus,
    pub updated_at: DateTime<Utc>,
}

impl RoomDailyState {
    pub fn new(room_id: Uuid, service_date: NaiveDate) -> Self {
        Self {
            room_id,
            service_date,
            occupancy_status: RoomDailyOccupancyStatus::Vacant,
            housekeeping_status: RoomDailyHousekeepingStatus::Inspected,
            updated_at: Utc::now(),
        }
    }

    pub fn set_occupancy_status(&mut self, status: RoomDailyOccupancyStatus) {
        self.occupancy_status = status;
        self.updated_at = Utc::now();
    }

    pub fn mark_dirty(&mut self) {
        self.housekeeping_status = RoomDailyHousekeepingStatus::Dirty;
        self.updated_at = Utc::now();
    }

    pub fn start_cleaning(&mut self) {
        self.housekeeping_status = RoomDailyHousekeepingStatus::Cleaning;
        self.updated_at = Utc::now();
    }

    pub fn finish_cleaning(&mut self) {
        self.housekeeping_status = RoomDailyHousekeepingStatus::Cleaned;
        self.updated_at = Utc::now();
    }

    pub fn inspect(&mut self) {
        self.housekeeping_status = RoomDailyHousekeepingStatus::Inspected;
        self.updated_at = Utc::now();
    }
}
