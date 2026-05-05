#[derive(Debug)]
pub struct HotelInventory {
    pub total_rooms: i32,
    pub reserved: i32,
}

impl HotelInventory {
    pub fn new(total_rooms: i32) -> Self {
        Self {
            total_rooms,
            reserved: 0,
        }
    }

    pub fn add_reservation(&mut self, rooms: i32) {
        self.reserved += rooms;
    }

    pub fn oversolved(&self) -> i32 {
        self.reserved - self.total_rooms
    }
}