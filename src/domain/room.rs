#[derive(Debug, Clone, PartialEq)]
pub enum OccupancyStatus {
    Vacant,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HousekeepingStatus {
    Dirty,
    Cleaning,
    Cleaned,
    Inspected,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub room_type: String,
    pub occupancy_status: OccupancyStatus,
    pub housekeeping_status: HousekeepingStatus,
}

impl Room {
    pub fn new(id: String, room_type: String) -> Self {
        Self {
            id,
            room_type,
            occupancy_status: OccupancyStatus::Vacant,
            housekeeping_status: HousekeepingStatus::Inspected,
        }
    }
}