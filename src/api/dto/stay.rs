use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Deserialize)]
pub struct AssignRoomRequest {
    pub room_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct StayResponse {
    pub id: Uuid,
    pub status: String,
}