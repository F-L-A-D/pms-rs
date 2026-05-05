use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HotelInventory {
    pub total_rooms: i32,
    pub reserved: HashMap<NaiveDate, i32>,
}

impl HotelInventory {
    pub fn new(total_rooms: i32) -> Self {
        Self {
            total_rooms,
            reserved: HashMap::new(),
        }
    }

    pub fn add_reservation(&mut self, date: NaiveDate, rooms: i32) {
        let entry = self.reserved.entry(date).or_insert(0);
        *entry += rooms;
    }

    pub fn oversolved(&self, date: NaiveDate) -> i32 {
        let reserved = self.reserved.get(&date).unwrap_or(&0);
        reserved - self.total_rooms
    }
}