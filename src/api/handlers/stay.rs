use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::api::error::map_app_error;
use crate::api::dto::stay::StayResponse;
use crate::api::state::AppState;

use crate::usecase::reservation::assign_room::assign_room;

use crate::usecase::stay::{
    check_in::check_in,
    check_out::check_out,
};

pub async fn assign_room_handler(
    State(state): State<AppState>,
    Path((reservation_id, room_id)): Path<(String, String)>,
) -> Result<Json<StayResponse>, StatusCode> {

    assign_room(
        &state.db,
        &reservation_id,
        &room_id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            StayResponse {
                id: reservation_id,
                status: "room_assigned".into(),
            }
        )
    )
}

pub async fn check_in_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StayResponse>, StatusCode> {

    check_in(
        &state.db,
        &id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            StayResponse {
                id,
                status: "checked_in".into(),
            }
        )
    )
}

pub async fn check_out_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StayResponse>, StatusCode> {

    check_out(
        &state.db,
        &id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            StayResponse {
                id,
                status: "checked_out".into(),
            }
        )
    )
}