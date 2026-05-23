use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use serde::Deserialize;

use uuid::Uuid;

use crate::{
    api::{
        dto::{
            input::room::{
                CreateRoomInput, GetRoomInput, ListRoomsInput,
                RoomDailyStateCommandInput, UpdateRoomActivationInput,
                UpdateRoomInput,
            },
            request::room::{
                CreateRoomRequest, RoomDailyStateCommandRequest,
                UpdateRoomActivationRequest, UpdateRoomRequest,
            },
            response::housekeeping::RoomDailyStateResponse,
            response::room::{RoomListResponse, RoomResponse},
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::room::{
        command::{
            create_room, mark_room_out_of_order, return_room_to_service,
            update_room::update_room,
            update_room_activation::update_room_activation,
        },
        detail::get_room::get_room,
        search::list_rooms::list_rooms,
    },
};

#[derive(Debug, Deserialize)]
pub struct ListRoomsQuery {
    pub service_date: Option<String>,
}

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

    let room = create_room::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

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

    let room = update_room(&state.db, input)
        .await
        .map_err(map_app_error)?;

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

pub async fn mark_room_out_of_order_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<RoomDailyStateCommandRequest>,
) -> Result<Json<RoomDailyStateResponse>, ApiError> {
    let input = room_daily_state_input(room_id, req.service_date)?;

    let state = mark_room_out_of_order::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(RoomDailyStateResponse::from(state)))
}

pub async fn return_room_to_service_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<RoomDailyStateCommandRequest>,
) -> Result<Json<RoomDailyStateResponse>, ApiError> {
    let input = room_daily_state_input(room_id, req.service_date)?;

    let state = return_room_to_service::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(RoomDailyStateResponse::from(state)))
}

pub async fn get_room_handler(
    State(state): State<AppState>,

    Path(room_id): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = GetRoomInput { room_id };

    let room = get_room(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = RoomResponse::from(room);

    Ok(Json(response))
}

pub async fn list_rooms_handler(
    State(state): State<AppState>,
    Query(query): Query<ListRoomsQuery>,
) -> Result<Json<RoomListResponse>, ApiError> {
    let service_date = parse_optional_service_date(query.service_date)?;

    let input = ListRoomsInput {
        include_inactive: false,
        service_date,
    };

    let rooms = list_rooms(&state.db, input)
        .await
        .map_err(map_app_error)?;

    let response = RoomListResponse::from(rooms);

    Ok(Json(response))
}

fn room_daily_state_input(
    room_id: String,
    service_date: String,
) -> Result<RoomDailyStateCommandInput, ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let service_date = NaiveDate::parse_from_str(&service_date, "%Y-%m-%d")
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(RoomDailyStateCommandInput {
        room_id,
        service_date,
    })
}

fn parse_optional_service_date(
    value: Option<String>,
) -> Result<Option<NaiveDate>, ApiError> {
    value
        .map(|value| {
            NaiveDate::parse_from_str(&value, "%Y-%m-%d")
                .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))
        })
        .transpose()
}