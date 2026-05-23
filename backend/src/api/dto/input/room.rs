use chrono::NaiveDate;

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateRoomInput {
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
}

#[derive(Debug, Clone)]
pub struct UpdateRoomInput {
    pub room_id: Uuid,
    pub room_no: Option<String>,
    pub room_class: Option<String>,
    pub capacity: Option<Option<u32>>,
    pub area_sqm: Option<Decimal>,
    pub is_physical: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct UpdateRoomActivationInput {
    pub room_id: Uuid,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct GetRoomInput {
    pub room_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct ListRoomsInput {
    pub include_inactive: bool,
    pub service_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct RoomDailyStateCommandInput {
    pub room_id: Uuid,
    pub service_date: NaiveDate,
}