use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::dto::stay::StayResponse;
use crate::api::error::{map_app_error, ApiError};
use crate::api::state::AppState;

use uuid::Uuid;

use crate::usecase::reservation::assign_room::assign_room;

use crate::usecase::stay::{check_in::check_in, check_out::check_out};

pub async fn assign_room_handler(
    State(state): State<AppState>,
    Path((reservation_id, room_id)): Path<(String, String)>,
) -> Result<Json<StayResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&reservation_id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    assign_room(&state.db, reservation_id, &room_id)
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

    check_in(&state.db, reservation_id)
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

    check_out(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(StayResponse {
        id: reservation_id,
        status: "checked_out".into(),
    }))
}
