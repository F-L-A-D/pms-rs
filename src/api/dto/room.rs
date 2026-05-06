use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub id: String,
    pub room_class: String,
}

#[derive(Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub room_class: String,
    pub occupancy_status: String,
    pub housekeeping_status: String,
}