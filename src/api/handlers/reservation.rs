use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    api::{
        dto::reservation::{
            CreateReservationRequest,
            ModifyReservationRequest,
            ReservationParticipantResponse,
            ReservationResponse,
        },
        error::{
            map_app_error,
            ApiError,
        },
        state::AppState,
    },
    domain::{
        reservation::{
            Reservation,
            ReservationStatus,
            StayStatus,
        },
        reservation_guest_relation::ReservationGuestRelation
    },

    usecase::reservation::{
        command::{
            cancel_reservation::cancel_reservation,
            create_reservation::create_reservation,
            modify_reservation::modify_reservation,
        },

        detail::get_reservation::get_reservation,

        search::get_guest_reservations::get_guest_reservations,
        
        stay_input::{
            StayInput,
            normalize,
        },
    },
};

pub async fn create_reservation_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<
    (StatusCode, Json<ReservationResponse>),
    ApiError,
> {

    let stay_input =
        StayInput::CheckInAndCheckOut {
            check_in: req.check_in,

            check_out: req.check_out,

            room_class: req.room_class.clone(),
        };

    let (check_in, check_out) =
        normalize(stay_input)
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e,
                )
            })?;

    let reservation_id = Uuid::new_v4();

    let participants =
        req.participants
            .into_iter()
            .map(|p| {
                ReservationGuestRelation {
                    reservation_id,

                    guest_id: p.guest_id,

                    relation_type: p.relation_type,
                }
            })
            .collect();

    let reservation =
        Reservation::new(
            reservation_id,

            req.external_id,

            check_in,

            check_out,

            req.room_class,

            participants,
        )
        .map_err(|e| {
            ApiError::new(
                axum::http::StatusCode::BAD_REQUEST,
                e,
            )
        })?;

    let response =
        reservation_to_response(
            reservation.clone(),
        );

    create_reservation(
        &state.db,
        reservation,
    )
    .await
    .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(response),
    ))
}

pub async fn modify_reservation_handler(
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

    let check_in =
        req.check_in
            .map(|v| {
                NaiveDate::parse_from_str(
                    &v,
                    "%Y-%m-%d",
                )
            })
            .transpose()
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let check_out =
        req.check_out
            .map(|v| {
                NaiveDate::parse_from_str(
                    &v,
                    "%Y-%m-%d",
                )
            })
            .transpose()
            .map_err(|e| {
                ApiError::new(
                    axum::http::StatusCode::BAD_REQUEST,
                    e.to_string(),
                )
            })?;

    let updated =
        modify_reservation(
            &state.db,

            reservation_id,

            check_in,

            check_out,

            req.room_class,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            reservation_to_response(
                updated,
            )
        )
    )
}

pub async fn cancel_reservation_handler(
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
        cancel_reservation(
            &state.db,
            reservation_id,
        )
        .await
        .map_err(map_app_error)?;

    Ok(
        Json(
            reservation_to_response(
                reservation,
            )
        )
    )
}

pub async fn get_reservation_handler(
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
                reservation,
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
                    reservation_to_response,
                )
                .collect(),
        )
    )
}

fn reservation_to_response(
    reservation: Reservation,
) -> ReservationResponse {

    ReservationResponse {
        id: reservation.id,

        external_id: reservation.external_id,

        check_in: reservation.check_in,

        check_out: reservation.check_out,

        reservation_status:
            match reservation.reservation_status {

                ReservationStatus::Active =>
                    "ACTIVE".into(),

                ReservationStatus::Cancelled =>
                    "CANCELLED".into(),
            },

        stay_status:
            reservation
                .stay_status
                .map(|s| {
                    match s {

                        StayStatus::Confirmed =>
                            "CONFIRMED".into(),

                        StayStatus::CheckedIn =>
                            "CHECKED_IN".into(),

                        StayStatus::CheckedOut =>
                            "CHECKED_OUT".into(),
                    }
                }),

        room_class: reservation.room_class,

        room_id: reservation.room_id,

        participants:
            reservation
                .participants
                .into_iter()
                .map(|p| {
                    ReservationParticipantResponse {
                        guest_id: p.guest_id,

                        relation_type:
                            p.relation_type
                                .as_str()
                                .to_string(),
                    }
                })
                .collect(),
    }
}