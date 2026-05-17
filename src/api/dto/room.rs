use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateRoomRequest {
    pub room_no: String,
    pub room_class: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
}
