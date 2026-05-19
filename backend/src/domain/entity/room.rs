use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
}

impl Room {
    pub fn new(
        id: Uuid,
        room_no: String,
        room_class: String,
        capacity: u32,
        area_sqm: Decimal,
    ) -> Self {
        Self {
            id,
            room_no,
            room_class,
            capacity: Some(capacity),
            area_sqm,
            is_physical: true,
            is_active: true,
        }
    }

    pub fn new_virtual(id: Uuid, room_no: String, room_class: String) -> Self {
        Self {
            id,
            room_no,
            room_class,
            capacity: None,
            area_sqm: Decimal::ZERO,
            is_physical: false,
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }
}
