use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use crate::adapter::stay_input::StayInput;

use crate::api::dto::reservation::{
    CreateReservationRequest,
    ModifyReservationRequest,
    ReservationResponse,
};

use crate::api::error::map_app_error;

use crate::api::state::AppState;

use crate::usecase::reservation::{
    cancel_reservation::cancel_reservation as cancel,
    create_reservation::create_reservation as create,
    modify_reservation::modify_reservation as modify,
};

pub async fn create_reservation(
    State(state): State<AppState>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, StatusCode> {

    let check_in =
        NaiveDate::parse_from_str(
            &req.check_in,
            "%Y-%m-%d",
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    create(
        &state.db,
        req.id.clone(),
        StayInput::CheckInAndNights {
            check_in,
            nights: req.nights as i64,
            room_class: req.room_class.clone(),
        },
        req.primary_guest_id.clone(),
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            ReservationResponse {
                id: req.id,
                status: "created".into(),
            }
        )
    )
}

pub async fn modify_reservation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ModifyReservationRequest>,
) -> Result<Json<ReservationResponse>, StatusCode> {

    let check_in =
        NaiveDate::parse_from_str(
            &req.check_in,
            "%Y-%m-%d",
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    modify(
        &state.db,
        &id,
        StayInput::CheckInAndNights {
            check_in,
            nights: req.nights as i64,
            room_class: req.room_class.clone(),
        }
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            ReservationResponse {
                id,
                status: "modified".into(),
            }
        )
    )
}

pub async fn cancel_reservation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, StatusCode> {

    cancel(
        &state.db,
        &id,
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            ReservationResponse {
                id,
                status: "cancelled".into(),
            }
        )
    )
}