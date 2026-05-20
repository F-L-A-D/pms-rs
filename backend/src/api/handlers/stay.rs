use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    api::{
        dto::{request::stay::MoveRoomRequest, response::stay::StayResponse},
        error::{map_app_error, ApiError},
        state::AppState,
    },
    usecase::{
        reservation::command::assign_room,
        stay::command::{check_in, check_out, move_room},
    },
};

pub async fn assign_room_handler(
    State(state): State<AppState>,
    Path((reservation_id, room_id)): Path<(String, String)>,
) -> Result<Json<StayResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&reservation_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    assign_room::execute(&state.db, reservation_id, room_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StayResponse {
        id: reservation_id,

        status: "room_assigned".into(),
    }))
}

pub async fn check_in_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StayResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    check_in::execute(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StayResponse {
        id: reservation_id,

        status: "checked_in".into(),
    }))
}

pub async fn check_out_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StayResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    check_out::execute(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StayResponse {
        id: reservation_id,

        status: "checked_out".into(),
    }))
}

pub async fn move_room_handler(
    State(state): State<AppState>,
    Path((reservation_id, room_id)): Path<(String, String)>,
    Json(req): Json<MoveRoomRequest>,
) -> Result<Json<StayResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&reservation_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    let room_id = Uuid::parse_str(&room_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    let effective_date = NaiveDate::parse_from_str(&req.effective_date, "%Y-%m-%d")
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    move_room::execute(&state.db, reservation_id, room_id, effective_date)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StayResponse {
        id: reservation_id,

        status: "room_moved".into(),
    }))
}
