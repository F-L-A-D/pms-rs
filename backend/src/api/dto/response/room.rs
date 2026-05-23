use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{
    api::dto::response::housekeeping::RoomDailyStateResponse,
    domain::{entity::room::Room, semantic::room_daily_state::RoomDailyState},
};

#[derive(Debug, Deserialize, Serialize)]
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
pub struct RoomListItemResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
    pub daily_state: Option<RoomDailyStateResponse>,
}

impl RoomListItemResponse {
    pub fn from_parts(room: Room, daily_state: Option<RoomDailyState>) -> Self {
        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            capacity: room.capacity,

            area_sqm: room.area_sqm,

            is_physical: room.is_physical,

            is_active: room.is_active,

            daily_state: daily_state.map(RoomDailyStateResponse::from),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoomListResponse {
    pub rooms: Vec<RoomListItemResponse>,
}

impl From<Vec<RoomListItemResponse>> for RoomListResponse {
    fn from(rooms: Vec<RoomListItemResponse>) -> Self {
        Self { rooms }
    }
}