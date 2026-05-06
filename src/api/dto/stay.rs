use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize)]
pub struct AssignRoomRequest {
    pub room_id: String,
}

#[derive(Serialize)]
pub struct StayResponse {
    pub id: String,
    pub status: String,
}