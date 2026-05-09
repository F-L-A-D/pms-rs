use axum::{extract::State, Json};

use crate::api::dto::room::{CreateRoomRequest, RoomResponse};

use crate::api::error::{map_app_error, ApiError};
use crate::api::state::AppState;

use crate::domain::room::Room;

use crate::usecase::room::{create_room::create_room, list_rooms::list_rooms};

pub async fn create_room_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room = Room::new(req.id.clone(), req.room_class.clone());

    create_room(&state.db, room.clone())
        .await
        .map_err(map_app_error)?;

    Ok(Json(RoomResponse {
        id: room.id,
        room_class: room.room_class,

        occupancy_status: format!("{:?}", room.occupancy_status),

        housekeeping_status: format!("{:?}", room.housekeeping_status),
    }))
}

pub async fn list_rooms_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<RoomResponse>>, ApiError> {
    let rooms = list_rooms(&state.db).await.map_err(map_app_error)?;

    let response = rooms
        .into_iter()
        .map(|room| RoomResponse {
            id: room.id,
            room_class: room.room_class,

            occupancy_status: format!("{:?}", room.occupancy_status),

            housekeeping_status: format!("{:?}", room.housekeeping_status),
        })
        .collect();

    Ok(Json(response))
}
