use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::usecase::reservation::stay_input::{normalize, StayInput};

use crate::api::dto::reservation::{
    CreateReservationRequest, ReservationParticipantResponse, ReservationResponse,
    ModifyReservationRequest,
};

use crate::api::error::{map_app_error, ApiError};

use crate::api::state::AppState;

use crate::domain::reservation::{Reservation, ReservationStatus, StayStatus};

use crate::domain::reservation_guest_relation::ReservationGuestRelation;

use crate::usecase::reservation::{
    cancel_reservation::cancel_reservation as cancel,
    create_reservation::create_reservation as create, 
    modify_reservation::modify_reservation as modify, 
    get_reservation::get_reservation,
    get_guest_reservations::get_guest_reservations,
};

pub async fn create_reservation(
    State(state): State<AppState>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let (check_in, check_out) = normalize(StayInput::CheckInAndCheckOut {
        check_in: req.check_in,
        check_out: req.check_out,
        room_class: req.room_class.clone(),
    })
    .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e))?;

    let reservation_id = Uuid::new_v4();

    let participants = req
        .participants
        .into_iter()
        .map(|p| ReservationGuestRelation {
            reservation_id: reservation_id,

            guest_id: p.guest_id,

            relation_type: p.relation_type,
        })
        .collect();

    let reservation = Reservation::new(
        reservation_id,
        req.external_id.clone(),
        check_in,
        check_out,
        req.room_class.clone(),
        participants,
    )
    .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e))?;

    create(&state.db, reservation.clone())
        .await
        .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn modify_reservation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ModifyReservationRequest>,
) -> Result<Json<ReservationResponse>, ApiError> {

    let reservation_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let reservation =
        get_reservation(
            &state.db,
            reservation_id,
        )
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| {
            ApiError::new(
                axum::http::StatusCode::NOT_FOUND,
                "reservation not found",
            )
        })?;

    let check_in =
        match req.check_in {

            Some(v) => {
                chrono::NaiveDate
                    ::parse_from_str(
                        &v,
                        "%Y-%m-%d",
                    )
                    .map_err(|e| {
                        ApiError::new(
                            axum::http::StatusCode
                                ::BAD_REQUEST,
                            e.to_string(),
                        )
                    })?
            }

            None => reservation.check_in,
        };

    let check_out =
        match req.check_out {

            Some(v) => {
                chrono::NaiveDate
                    ::parse_from_str(
                        &v,
                        "%Y-%m-%d",
                    )
                    .map_err(|e| {
                        ApiError::new(
                            axum::http::StatusCode
                                ::BAD_REQUEST,
                            e.to_string(),
                        )
                    })?
            }

            None => reservation.check_out,
        };

    let room_class =
        req.room_class
            .unwrap_or(
                reservation.room_class
            );

    modify(
        &state.db,
        reservation_id,

        StayInput::CheckInAndCheckOut {
            check_in,
            check_out,
            room_class,
        },
    )
    .await
    .map_err(map_app_error)?;

    let updated =
        get_reservation(
            &state.db,
            reservation_id,
        )
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| {
            ApiError::new(
                axum::http::StatusCode::NOT_FOUND,
                "reservation not found",
            )
        })?;

    Ok(
        Json(
            reservation_to_response(
                updated
            )
        )
    )
}

pub async fn cancel_reservation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()))?;

    cancel(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?;

    let reservation =
        crate::usecase::reservation::get_reservation::get_reservation(&state.db, reservation_id)
            .await
            .map_err(map_app_error)?
            .ok_or_else(|| {
                ApiError::new(axum::http::StatusCode::NOT_FOUND, "reservation not found")
            })?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn get_reservation_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {

    let reservation_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let reservation =
        get_reservation(
            &state.db,
            reservation_id,
        )
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| {
            ApiError::new(
                axum::http::StatusCode::NOT_FOUND,
                "reservation not found",
            )
        })?;

    Ok(
        Json(
            reservation_to_response(
                reservation
            )
        )
    )
}

pub async fn get_guest_reservations_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<
    Json<Vec<ReservationResponse>>,
    ApiError,
> {

    let guest_id =
        Uuid::parse_str(&id)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let reservations =
        get_guest_reservations(
            &state.db,
            guest_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            reservations
                .into_iter()
                .map(
                    reservation_to_response
                )
                .collect()
        )
    )
}

fn reservation_to_response(reservation: Reservation) -> ReservationResponse {
    ReservationResponse {
        id: reservation.id,

        external_id: reservation.external_id,

        check_in: reservation.check_in,

        check_out: reservation.check_out,

        reservation_status: match reservation.reservation_status {
            ReservationStatus::Active => "ACTIVE".into(),

            ReservationStatus::Cancelled => "CANCELLED".into(),
        },

        stay_status: reservation.stay_status.map(|s| match s {
            StayStatus::Confirmed => "CONFIRMED".into(),

            StayStatus::CheckedIn => "CHECKED_IN".into(),

            StayStatus::CheckedOut => "CHECKED_OUT".into(),
        }),

        room_class: reservation.room_class,

        room_id: reservation.room_id,

        participants: reservation
            .participants
            .into_iter()
            .map(|p| ReservationParticipantResponse {
                guest_id: p.guest_id,

                relation_type: p.relation_type.as_str().to_string(),
            })
            .collect(),
    }
}
