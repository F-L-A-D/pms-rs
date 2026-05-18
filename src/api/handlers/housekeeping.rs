use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    api::{
        dto::{
            input::housekeeping::HousekeepingRoomDailyStateInput,
            request::housekeeping::HousekeepingRoomDailyStateRequest,
            response::housekeeping::RoomDailyStateResponse,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::housekeeping::command::{finish_cleaning, inspect_room, mark_dirty, start_cleaning},
};

pub async fn mark_dirty_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<HousekeepingRoomDailyStateRequest>,
) -> Result<(StatusCode, Json<RoomDailyStateResponse>), ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let service_date = parse_service_date(req.service_date)?;

    let state = mark_dirty::execute(
        &state.db,
        HousekeepingRoomDailyStateInput {
            room_id,
            service_date,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(RoomDailyStateResponse::from(state))))
}

pub async fn start_cleaning_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<HousekeepingRoomDailyStateRequest>,
) -> Result<(StatusCode, Json<RoomDailyStateResponse>), ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let service_date = parse_service_date(req.service_date)?;

    let state = start_cleaning::execute(
        &state.db,
        HousekeepingRoomDailyStateInput {
            room_id,
            service_date,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(RoomDailyStateResponse::from(state))))
}

pub async fn finish_cleaning_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<HousekeepingRoomDailyStateRequest>,
) -> Result<(StatusCode, Json<RoomDailyStateResponse>), ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let service_date = parse_service_date(req.service_date)?;

    let state = finish_cleaning::execute(
        &state.db,
        HousekeepingRoomDailyStateInput {
            room_id,
            service_date,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(RoomDailyStateResponse::from(state))))
}

pub async fn inspect_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(req): Json<HousekeepingRoomDailyStateRequest>,
) -> Result<(StatusCode, Json<RoomDailyStateResponse>), ApiError> {
    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let service_date = parse_service_date(req.service_date)?;

    let state = inspect_room::execute(
        &state.db,
        HousekeepingRoomDailyStateInput {
            room_id,
            service_date,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::OK, Json(RoomDailyStateResponse::from(state))))
}

fn parse_service_date(value: String) -> Result<NaiveDate, ApiError> {
    NaiveDate::parse_from_str(&value, "%Y-%m-%d")
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))
}
