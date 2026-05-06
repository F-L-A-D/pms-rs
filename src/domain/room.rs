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
    pub room_class: String,
    pub occupancy_status: OccupancyStatus,
    pub housekeeping_status: HousekeepingStatus,
}

impl Room {
    pub fn new(id: String, room_class: String) -> Self {
        Self {
            id,
            room_class,
            occupancy_status: OccupancyStatus::Vacant,
            housekeeping_status: HousekeepingStatus::Inspected,
        }
    }
    
    pub fn check_in(&mut self) -> Result<(), String> {
        match self.occupancy_status {
            OccupancyStatus::Vacant => {
                if self.housekeeping_status != HousekeepingStatus::Inspected {
                    return Err("room not ready".into());
                }

                self.occupancy_status = OccupancyStatus::Occupied;
                Ok(())
            }
            OccupancyStatus::Occupied => {
                Err("already occupied".into())
            }
        }
    }

    pub fn check_out(&mut self) -> Result<(), String> {
        match self.occupancy_status {
            OccupancyStatus::Occupied => {
                self.occupancy_status = OccupancyStatus::Vacant;
                self.housekeeping_status = HousekeepingStatus::Dirty;
                Ok(())
            }
            OccupancyStatus::Vacant => {
                Err("room is already vacant".into())
            }
        }
    }

    pub fn mark_dirty(&mut self) -> Result<(), String> {
        self.housekeeping_status = 
            HousekeepingStatus::Dirty;
        
        Ok(())
    }

    pub fn start_cleaning(&mut self) -> Result<(), String> {
        match self.housekeeping_status {
            HousekeepingStatus::Dirty => {
                self.housekeeping_status = HousekeepingStatus::Cleaning;
                Ok(())
            }
            _ => Err("invalid cleaning transition".into()),
        }
    }

    pub fn finish_cleaning(&mut self) -> Result<(), String> {
        match self.housekeeping_status {
            HousekeepingStatus::Cleaning => {
                self.housekeeping_status = HousekeepingStatus::Cleaned;
                Ok(())
            }
            _ => Err("invalid cleaning transition".into()),
        }
    }

    pub fn inspect(&mut self) -> Result<(), String> {
        match self.housekeeping_status {
            HousekeepingStatus::Cleaned => {
                self.housekeeping_status = HousekeepingStatus::Inspected;
                Ok(())
            }
            _ => Err("invalid inspection transition".into()),
        }
    }
}