use crate::{
    api::dto::response::room::RoomAssignmentVisibilityResponse,
    domain::entity::reservation::{Reservation, ReservationStatus, StayStatus},
};

pub fn build_assignment_visibility(
    reservations: Vec<Reservation>,
) -> RoomAssignmentVisibilityResponse {
    let mut warning_candidate: Option<Reservation> = None;

    for reservation in reservations {
        if reservation.reservation_status == ReservationStatus::Cancelled {
            continue;
        }

        if reservation.stay_status == Some(StayStatus::CheckedOut) {
            continue;
        }

        if reservation.reservation_status == ReservationStatus::NoShow
            || reservation.stay_status == Some(StayStatus::NoShow)
        {
            if warning_candidate.is_none() {
                warning_candidate = Some(reservation);
            }

            continue;
        }

        if reservation.reservation_status == ReservationStatus::Confirmed
            && matches!(
                reservation.stay_status,
                Some(StayStatus::Confirmed) | Some(StayStatus::CheckedIn)
            )
        {
            return RoomAssignmentVisibilityResponse::assigned(reservation);
        }
    }

    if let Some(reservation) = warning_candidate {
        return RoomAssignmentVisibilityResponse::no_show_warning(reservation);
    }

    RoomAssignmentVisibilityResponse::unassigned()
}
