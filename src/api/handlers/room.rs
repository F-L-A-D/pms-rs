use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{
            input::room::{
                CreateRoomInput, GetRoomInput, ListRoomsInput, UpdateRoomActivationInput,
                UpdateRoomInput,
            },
            request::room::{CreateRoomRequest, UpdateRoomActivationRequest, UpdateRoomRequest},
            response::room::{RoomListResponse, RoomResponse},
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::room::{
        command::{
            create_room::create_room, update_room::update_room,
            update_room_activation::update_room_activation,
        },
        detail::get_room::get_room,
        search::list_rooms::list_rooms,
    },
};

pub async fn create_room_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<RoomResponse>), ApiError> {
    let input = CreateRoomInput {
        room_no: req.room_no,

        room_class: req.room_class,

        capacity: req.capacity,

        area_sqm: req.area_sqm,

        is_physical: req.is_physical,
    };

    let room = create_room(&state.db, input).await.map_err(map_app_error)?;

    let response = RoomResponse::from(room);

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn update_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<UpdateRoomRequest>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = UpdateRoomInput {
        room_id,

        room_no: req.room_no,

        room_class: req.room_class,

        capacity: req.capacity,

        area_sqm: req.area_sqm,

        is_physical: req.is_physical,
    };

    let room = update_room(&state.db, input).await.map_err(map_app_error)?;

    let response = RoomResponse::from(room);

    Ok(Json(response))
}

pub async fn update_room_activation_handler(
    State(state): State<AppState>,

    Path(room_id): Path<String>,

    Json(req): Json<UpdateRoomActivationRequest>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = UpdateRoomActivationInput {
        room_id,

        is_active: req.is_active,
    };

    let room = update_room_activation(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = RoomResponse::from(room);

    Ok(Json(response))
}

pub async fn get_room_handler(
    State(state): State<AppState>,

    Path(room_id): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = GetRoomInput { room_id };

    let room = get_room(&state.db, input).await.map_err(map_app_error)?;

    let response = RoomResponse::from(room);

    Ok(Json(response))
}

pub async fn list_rooms_handler(
    State(state): State<AppState>,
) -> Result<Json<RoomListResponse>, ApiError> {
    let input = ListRoomsInput {
        include_inactive: false,
    };

    let rooms = list_rooms(&state.db, input).await.map_err(map_app_error)?;

    let response = RoomListResponse::from(rooms);

    Ok(Json(response))
}
