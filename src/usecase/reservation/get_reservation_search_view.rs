use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    reservation_repository::
    SqliteReservationRepository;

use crate::repository::sqlite::operational::
    guest_repository::
    SqliteGuestRepository;

use crate::usecase::reservation::
    reservation_search_view::
    ReservationSearchView;

pub async fn get_reservation_search_view(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
) -> AppResult<
    ReservationSearchView,
> {

    let reservation =
        SqliteReservationRepository
            ::find_by_id(
                tx,
                reservation_id,
            )
            .await
            .map_err(
                AppError::Infrastructure,
            )?
            .ok_or(
                AppError::NotFound(
                    "reservation not found"
                        .into(),
                ),
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
                    AppError::NotFound(
                        "guest not found"
                            .into(),
                    ),
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
        ReservationSearchView {
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
                reservation.reservation_status,

            stay_status:
                reservation.stay_status,
        }
    )
}