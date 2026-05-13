use uuid::Uuid;

use crate::domain::room::Room;

use serde::{
    Deserialize,
    Serialize,
};

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
    pub occupancy_status: String,
    pub housekeeping_status: String,
}

impl From<Room> for RoomResponse {

    fn from(room: Room) -> Self {

        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            occupancy_status:
                format!("{:?}", room.occupancy_status),

            housekeeping_status:
                format!("{:?}", room.housekeeping_status),
        }
    }
}