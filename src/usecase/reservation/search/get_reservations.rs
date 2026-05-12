use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    api::dto::reservation::ReservationSearchResponse,

    error::app_error::{
        AppResult,
        not_found,
    },

    repository::sqlite::operational::{
        guest_repository::
            SqliteGuestRepository,

        reservation_repository::
            SqliteReservationRepository,
    },
};

pub async fn get_reservation_search_result(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
) -> AppResult<ReservationSearchResponse> {

    let reservation =
        SqliteReservationRepository
            ::find_by_id(
                tx,
                reservation_id,
            )
            .await?
            .ok_or(
                not_found(
                    "reservation not found"
                )
            )?;

    let mut participant_names =
        vec![];

    let mut primary_guest_name =
        String::new();

    for participant in
        &reservation.participants
    {

        let guest =
            SqliteGuestRepository
                ::find_by_id(
                    tx,
                    participant.guest_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "guest not found"
                    )
                )?;

        let full_name =
            format!(
                "{} {}",
                guest.last_name,
                guest.first_name,
            );

        if participant.is_primary() {

            primary_guest_name =
                full_name.clone();
        }

        participant_names
            .push(full_name);
    }

    Ok(
        ReservationSearchResponse {

            reservation_id:
                reservation.id,

            external_id:
                reservation.external_id,

            primary_guest_name,

            participant_names,

            check_in:
                reservation.check_in,

            check_out:
                reservation.check_out,

            room_class:
                reservation.room_class,

            room_id: 
                reservation.room_id,

            reservation_status:
                format!(
                    "{:?}",
                    reservation
                        .reservation_status
                ),

            stay_status:
                reservation
                    .stay_status
                    .map(
                        |s| format!("{:?}", s)
                    ),
        }
    )
}