use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entity::room::Room;

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            capacity: room.capacity,

            area_sqm: room.area_sqm,

            is_physical: room.is_physical,

            is_active: room.is_active,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoomListResponse {
    pub rooms: Vec<RoomResponse>,
}

impl From<Vec<Room>> for RoomListResponse {
    fn from(rooms: Vec<Room>) -> Self {
        Self {
            rooms: rooms.into_iter().map(RoomResponse::from).collect(),
        }
    }
}
