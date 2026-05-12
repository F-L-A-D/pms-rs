use axum::{
    extract::{Path, State},
    Json,
};

use serde_json::json;

use uuid::Uuid;

use crate::{
    api::{
        error::{
            map_app_error,
            ApiError,
        },

        state::AppState,
    },

    usecase::housekeeping::command::{
        finish_cleaning::finish_cleaning,
        inspect_room::inspect_room,
        mark_dirty::mark_dirty,
        start_cleaning::start_cleaning,
    },
};

pub async fn mark_dirty_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {

    let room_id =
        Uuid::parse_str(&room_id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    mark_dirty(
        &state.db,
        room_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            json!({
                "room_id": room_id,
                "status": "dirty"
            })
        )
    )
}

pub async fn start_cleaning_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {

    let room_id =
        Uuid::parse_str(&room_id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    start_cleaning(
        &state.db,
        room_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            json!({
                "room_id": room_id,
                "status": "cleaning"
            })
        )
    )
}

pub async fn finish_cleaning_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {

    let room_id =
        Uuid::parse_str(&room_id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    finish_cleaning(
        &state.db,
        room_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            json!({
                "room_id": room_id,
                "status": "cleaned"
            })
        )
    )
}

pub async fn inspect_room_handler(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {

    let room_id =
        Uuid::parse_str(&room_id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    inspect_room(
        &state.db,
        room_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            json!({
                "room_id": room_id,
                "status": "inspected"
            })
        )
    )
}