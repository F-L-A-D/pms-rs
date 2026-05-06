use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::api::dto::room::{
    CreateRoomRequest,
    RoomResponse,
};

use crate::api::state::AppState;

use crate::domain::room::Room;


use crate::repository::sqlite::room_repository::SqliteRoomRepository;

pub async fn create_room(
    State(state): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, StatusCode> {

    let room = Room::new(
        req.id.clone(),
        req.room_class.clone(),
    );

    SqliteRoomRepository::save(
        &state.db.pool,
        &room,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            RoomResponse {
                id: room.id,
                room_class: room.room_class,
                occupancy_status:
                    format!("{:?}", room.occupancy_status),

                housekeeping_status:
                    format!("{:?}", room.housekeeping_status),
            }
        )
    )
}

pub async fn list_rooms(
    State(state): State<AppState>,
) -> Result<Json<Vec<RoomResponse>>, StatusCode> {

    let rooms =
        SqliteRoomRepository::find_all(
            &state.db.pool,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response =
        rooms
            .into_iter()
            .map(|room| {
                RoomResponse {
                    id: room.id,
                    room_class: room.room_class,

                    occupancy_status:
                        format!("{:?}", room.occupancy_status),

                    housekeeping_status:
                        format!("{:?}", room.housekeeping_status),
                }
            })
            .collect();

    Ok(Json(response))
}