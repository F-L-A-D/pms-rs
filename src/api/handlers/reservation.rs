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
                CloseReservationEditSessionInput, CreateReservationInput, ModifyReservationInput,
                OpenReservationEditSessionInput, ReservationDailyDetailInput,
                ReservationDailyRevenueAllocationInput, ReservationPackageBreakdownInput,
                ReservationParticipantInput,
            },
            request::reservation::{
                CloseReservationEditSessionRequest, CreateReservationRequest,
                ModifyReservationRequest, OpenReservationEditSessionRequest,
            },
            response::reservation::{
                OpenReservationEditSessionResponse, ReservationEditSessionResponse,
                ReservationEditSessionWarningResponse, ReservationParticipantResponse,
                ReservationResponse,
            },
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    domain::{
        entity::reservation::Reservation,
        semantic::{
            operation_context::OperationContext, reservation_booking::ReservationBookingChannel,
            reservation_edit_session::ReservationEditSession,
        },
    },
    usecase::reservation::{
        command::{
            cancel_reservation::cancel_reservation, close_edit_session, create_reservation,
            mark_no_show, modify_reservation, open_edit_session, reinstate_reservation,
        },
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

    let daily_details = req
        .daily_details
        .into_iter()
        .map(|detail| {
            let service_date = NaiveDate::parse_from_str(&detail.service_date, "%Y-%m-%d")
                .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

            let package_breakdowns = detail
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

            Ok(ReservationDailyDetailInput {
                service_date,
                room_class: detail.room_class,
                plan_code: detail.plan_code,
                adult_count: detail.adult_count,
                child_count: detail.child_count,
                package_breakdowns,
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

        daily_details,

        participants,
    };

    let reservation = create_reservation::execute(&state.db, input, OperationContext::api_system())
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

    let package_breakdowns = req
        .package_breakdowns
        .map(|breakdowns| {
            breakdowns
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
                .collect::<Result<Vec<_>, ApiError>>()
        })
        .transpose()?;

    let daily_details = req
        .daily_details
        .map(|details| {
            details
                .into_iter()
                .map(|detail| {
                    let service_date = NaiveDate::parse_from_str(&detail.service_date, "%Y-%m-%d")
                        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

                    let package_breakdowns = detail
                        .package_breakdowns
                        .into_iter()
                        .map(|breakdown| {
                            let amount = breakdown.amount.parse::<Decimal>().map_err(|e| {
                                ApiError::new(StatusCode::BAD_REQUEST, e.to_string())
                            })?;

                            Ok(ReservationPackageBreakdownInput {
                                package_code: breakdown.package_code,
                                revenue_category: breakdown.revenue_category,
                                amount,
                            })
                        })
                        .collect::<Result<Vec<_>, ApiError>>()?;

                    Ok(ReservationDailyDetailInput {
                        service_date,
                        room_class: detail.room_class,
                        plan_code: detail.plan_code,
                        adult_count: detail.adult_count,
                        child_count: detail.child_count,
                        package_breakdowns,
                    })
                })
                .collect::<Result<Vec<_>, ApiError>>()
        })
        .transpose()?;

    let daily_revenue_allocations = req
        .daily_revenue_allocations
        .map(|allocations| {
            allocations
                .into_iter()
                .map(|allocation| {
                    let service_date =
                        NaiveDate::parse_from_str(&allocation.service_date, "%Y-%m-%d")
                            .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
                    let amount = allocation
                        .amount
                        .parse::<Decimal>()
                        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

                    Ok(ReservationDailyRevenueAllocationInput {
                        service_date,
                        package_code: allocation.package_code,
                        revenue_category: allocation.revenue_category,
                        department_code: allocation.department_code,
                        account_code: allocation.account_code,
                        amount,
                    })
                })
                .collect::<Result<Vec<_>, ApiError>>()
        })
        .transpose()?;

    let participants = req
        .participants
        .map(|participants| {
            participants
                .into_iter()
                .map(|participant| {
                    let guest_id = Uuid::parse_str(&participant.guest_id)
                        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

                    Ok(ReservationParticipantInput {
                        guest_id,
                        relation_type: participant.relation_type,
                    })
                })
                .collect::<Result<Vec<_>, ApiError>>()
        })
        .transpose()?;

    let input = ModifyReservationInput {
        expected_version: req.expected_version,

        check_in,
        check_out,

        room_class: req.room_class,
        package_breakdowns,
        daily_details,
        daily_revenue_allocations,
        participants,
    };

    let updated = modify_reservation::execute(
        &state.db,
        reservation_id,
        input,
        OperationContext::api_system(),
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(updated)))
}

pub async fn open_reservation_edit_session_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<OpenReservationEditSessionRequest>,
) -> Result<Json<OpenReservationEditSessionResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let result = open_edit_session::execute(
        &state.db,
        OpenReservationEditSessionInput {
            reservation_id,
            actor_id: req.actor_id,
            actor_label: req.actor_label,
            lease_minutes: req.lease_minutes,
        },
    )
    .await
    .map_err(map_app_error)?;

    let warning = if result.active_other_sessions.is_empty() {
        None
    } else {
        Some(ReservationEditSessionWarningResponse {
            active_sessions: result
                .active_other_sessions
                .into_iter()
                .map(reservation_edit_session_to_response)
                .collect(),
        })
    };

    Ok(Json(OpenReservationEditSessionResponse {
        session: reservation_edit_session_to_response(result.session),
        warning,
    }))
}

pub async fn close_reservation_edit_session_handler(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(req): Json<CloseReservationEditSessionRequest>,
) -> Result<StatusCode, ApiError> {
    let session_id = Uuid::parse_str(&session_id)
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    close_edit_session::execute(
        &state.db,
        CloseReservationEditSessionInput {
            session_id,
            actor_id: req.actor_id,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn cancel_reservation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation = cancel_reservation(&state.db, reservation_id, OperationContext::api_system())
        .await
        .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn mark_no_show_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation =
        mark_no_show::execute(&state.db, reservation_id, OperationContext::api_system())
            .await
            .map_err(map_app_error)?;

    Ok(Json(reservation_to_response(reservation)))
}

pub async fn reinstate_reservation_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ReservationResponse>, ApiError> {
    let reservation_id =
        Uuid::parse_str(&id).map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, e.to_string()))?;

    let reservation =
        reinstate_reservation::execute(&state.db, reservation_id, OperationContext::api_system())
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

fn reservation_edit_session_to_response(
    session: ReservationEditSession,
) -> ReservationEditSessionResponse {
    ReservationEditSessionResponse {
        id: session.id,
        reservation_id: session.reservation_id,
        actor_id: session.actor_id,
        actor_label: session.actor_label,
        opened_at: session.opened_at,
        expires_at: session.expires_at,
    }
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

        version: reservation.version,

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

        daily_details: reservation
            .daily_stay_details
            .into_iter()
            .map(
                |detail| crate::api::dto::response::reservation::ReservationDailyDetailResponse {
                    service_date: detail.service_date,
                    room_class: detail.room_class,
                    plan_code: detail.plan_code,
                    adult_count: detail.adult_count,
                    child_count: detail.child_count,
                },
            )
            .collect(),

        daily_revenue_allocations: reservation
            .daily_revenue_allocations
            .into_iter()
            .map(|allocation| {
                crate::api::dto::response::reservation::ReservationDailyRevenueAllocationResponse {
                    service_date: allocation.service_date,
                    package_code: allocation.package_code,
                    revenue_category: allocation.revenue_category,
                    department_code: allocation.department_code,
                    account_code: allocation.account_code,
                    amount: allocation.amount,
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
