use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssignRoomRequest {
    pub room_id: String,
}

#[derive(Deserialize)]
pub struct MoveRoomRequest {
    pub effective_date: String,
}
