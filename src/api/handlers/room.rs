use axum::{
    extract::State,
    Json,
};

use uuid::Uuid;

use crate::{
    api::{
        dto::room::{
            CreateRoomRequest,
            RoomResponse,
        },

        error::{
            map_app_error,
            ApiError,
        },

        state::AppState,
    },

    domain::room::Room,

    usecase::room::{
        command::create_room::create_room,

        search::get_rooms::get_rooms,
    },
};

pub async fn create_room_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, ApiError> {

    let room =
        Room::new(
            Uuid::new_v4(),
            req.room_no.clone(),
            req.room_class.clone(),
        );

    let response =
        RoomResponse::from(
            room.clone()
    );

    create_room(
        &state.db,
        room,
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(response))
}

pub async fn get_rooms_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<RoomResponse>>, ApiError> {

    let rooms =
        get_rooms(&state.db)
            .await
            .map_err(map_app_error)?;

    let response =
        rooms
            .into_iter()
            .map(|room| {
                RoomResponse::from(
                    room.clone()
                )
            })
            .collect();

    Ok(Json(response))
}