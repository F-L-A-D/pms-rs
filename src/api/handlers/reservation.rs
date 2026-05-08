use axum::{
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    Json,
};

use crate::usecase::reservation::stay_input::{
    normalize,
    StayInput,
};

use crate::api::dto::reservation::{
    CreateReservationRequest,
    UpdateReservationRequest,
    ReservationParticipantResponse,
    ReservationResponse,
};

use crate::api::error::map_app_error;

use crate::api::state::AppState;

use crate::domain::reservation::{
    Reservation,
    ReservationStatus,
    StayStatus,
};

use crate::domain::reservation_guest_relation::{
    ReservationGuestRelation,
};

use crate::usecase::reservation::{
    cancel_reservation::cancel_reservation as cancel,
    create_reservation::create_reservation as create,
    modify_reservation::modify_reservation as modify,
    get_reservation::get_reservation,
};

pub async fn create_reservation(
    State(state): State<AppState>,
    Json(req): Json<CreateReservationRequest>,
) -> Result<Json<ReservationResponse>, StatusCode> {

    let (check_in, check_out) =
        normalize(
            StayInput::CheckInAndCheckOut {
                check_in: req.check_in,
                check_out: req.check_out,
                room_class:
                    req.room_class.clone(),
            }
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let participants =
        req.participants
            .into_iter()
            .map(|p| {

                ReservationGuestRelation {
                    reservation_id:
                        req.id.clone(),

                    guest_id:
                        p.guest_id,

                    relation_type:
                        p.relation_type,
                }
            })
            .collect();

    let reservation =
        Reservation::new(
            req.id.clone(),
            check_in,
            check_out,
            req.room_class.clone(),
            participants,
        )
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    create(
        &state.db,
        reservation.clone(),
    )
    .await
    .map_err(map_app_error)?;

    Ok(
        Json(
            reservation_to_response(
                reservation
            )
        )
    )
}

pub async fn modify_reservation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateReservationRequest>,
) -> Result<Json<ReservationResponse>, StatusCode> {

    modify(
        &state.db,
        &id,
        StayInput::CheckInAndCheckOut {
            check_in: req.check_in,
            check_out: req.check_out,
            room_class:
                req.room_class.clone(),
        }
    )
    .await
    .map_err(map_app_error)?;

    let reservation =
       get_reservation(
            &state.db,
            &id,
        )
        .await
        .map_err(map_app_error)?
        .ok_or(
            StatusCode::NOT_FOUND
        )?;

    Ok(
        Json(
            reservation_to_response(
                reservation
            )
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

    let reservation =
        crate::usecase::reservation::get_reservation
            ::get_reservation(
                &state.db,
                &id,
            )
            .await
            .map_err(map_app_error)?
            .ok_or(
                StatusCode::NOT_FOUND
            )?;

    Ok(
        Json(
            reservation_to_response(
                reservation
            )
        )
    )
}

fn reservation_to_response(
    reservation: Reservation,
) -> ReservationResponse {

    ReservationResponse {

        id:
            reservation.id,

        check_in:
            reservation.check_in,

        check_out:
            reservation.check_out,

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

        room_class:
            reservation.room_class,

        room_id:
            reservation.room_id,

        participants:
            reservation
                .participants
                .into_iter()
                .map(|p| {

                    ReservationParticipantResponse {

                        guest_id:
                            p.guest_id,

                        relation_type:
                            p.relation_type
                                .as_str()
                                .to_string(),
                    }
                })
                .collect(),
    }
}