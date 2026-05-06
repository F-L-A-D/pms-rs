use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::api::dto::stay::{
    AssignRoomRequest,
    StayResponse,
};

use crate::api::state::AppState;

use crate::usecase::reservation::assign_room::assign_room;

use crate::usecase::stay::{
    check_in::check_in,
    check_out::check_out,
};

pub async fn assign_room_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<AssignRoomRequest>,
) -> Result<Json<StayResponse>, StatusCode> {

    assign_room(
        &state.db,
        &id,
        &req.room_id,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            StayResponse {
                id,
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(
        Json(
            StayResponse {
                id,
                status: "checked_out".into(),
            }
        )
    )
}