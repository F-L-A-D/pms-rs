use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::NaiveDate;

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::{
        dto::{
            input::reservation::{
                CreateReservationInput, ModifyReservationInput, ReservationPackageBreakdownInput,
                ReservationParticipantInput,
            },
            request::reservation::{CreateReservationRequest, ModifyReservationRequest},
            response::reservation::{ReservationParticipantResponse, ReservationResponse},
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    domain::entity::reservation::Reservation,
    domain::semantic::reservation_booking::ReservationBookingChannel,
    usecase::reservation::{
        command::{cancel_reservation::cancel_reservation, create_reservation, modify_reservation},
        detail::get_reservation::get_reservation,
        search::get_guest_reservations::get_guest_reservations,
    },
};

pub async fn create_reservation_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<(StatusCode, Json<ReservationResponse>), ApiError> {
    let check_in = NaiveDate::parse_from_str(&req.check_in, "%Y-%m-%d")
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let check_out = NaiveDate::parse_from_str(&req.check_out, "%Y-%m-%d")
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let participants = req
        .participants
        .into_iter()
        .map(|p| {
            let guest_id = Uuid::parse_str(&p.guest_id)
                .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

            Ok(ReservationParticipantInput {
                guest_id,

                relation_type: p.relation_type,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;

    let package_breakdowns = req
        .package_breakdowns
        .into_iter()
        .map(|breakdown| {
            let amount = breakdown
                .amount
                .parse::<Decimal>()
                .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

            Ok(ReservationPackageBreakdownInput {
                package_code: breakdown.package_code,
                revenue_category: breakdown.revenue_category,
                amount,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;

    let input = CreateReservationInput {
        external_id: req.external_id,

        check_in,

        check_out,

        room_class: req.room_class,

        booking_channel: req
            .booking_channel
            .unwrap_or(ReservationBookingChannel::Direct),

        plan_code: req.plan_code,

        package_breakdowns,

        participants,
    };

    let reservation = create_reservation::execute(&state.db, input)
        .await
        .map_err(map_app_error)?;

    Ok((
        StatusCode::CREATED,
        Json(reservation_to_response(reservation)),
    ))
}

pub async fn modify_reservation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ModifyReservationRequest>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let check_in = req
        .check_in
        .map(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let check_out = req
        .check_out
        .map(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let input = ModifyReservationInput {
        check_in,
        check_out,

        room_class: req.room_class,
    };

    let updated = modify_reservation::execute(&state.db, reservation_id, input)
        .await
        .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(updated)))
}

pub async fn cancel_reservation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation = cancel_reservation(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn get_reservation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation = get_reservation(&state.db, reservation_id)
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| ApiError::new(StatusCode::NOT_FOUND, "reservation not found"))?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn get_guest_reservations_handler(
    State(state): State<AppState>,
    Path(guest_id): Path<String>,
) -> Result<Json<Vec<ReservationResponse>>, ApiError> {
    let guest_id = Uuid::parse_str(&guest_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservations = get_guest_reservations(&state.db, guest_id)
        .await
        .map_err(map_app_error)?;

    Ok(Json(
        reservations
            .into_iter()
            .map(reservation_to_response)
            .collect(),
    ))
}

fn reservation_to_response(reservation: Reservation) -> ReservationResponse {
    ReservationResponse {
        id: reservation.id,

        external_id: reservation.external_id,

        check_in: reservation.check_in,

        check_out: reservation.check_out,

        reservation_status: reservation.reservation_status,

        stay_status: reservation.stay_status,

        room_class: reservation.room_class,

        room_id: reservation.room_id,

        booking_channel: reservation.booking_channel,

        plan_code: reservation.plan_code,

        package_breakdowns: reservation
            .package_breakdowns
            .into_iter()
            .map(|breakdown| {
                crate::api::dto::response::reservation::ReservationPackageBreakdownResponse {
                    package_code: breakdown.package_code,
                    revenue_category: breakdown.revenue_category,
                    amount: breakdown.amount,
                }
            })
            .collect(),

        participants: reservation
            .participants
            .into_iter()
            .map(|p| ReservationParticipantResponse {
                guest_id: p.guest_id,

                relation_type: p.relation_type,
            })
            .collect(),
    }
}
