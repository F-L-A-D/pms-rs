use uuid::Uuid;

use crate::db::connection::Db;

use crate::domain::reservation::Reservation;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    reservation_repository::
    SqliteReservationRepository;

pub async fn get_guest_reservations(
    db: &Db,
    guest_id: Uuid,
) -> AppResult<Vec<Reservation>> {

    let mut tx =
        db.begin_tx().await;

    let reservations =
        SqliteReservationRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .map_err(
                AppError::Infrastructure
            )?;

    Ok(
        reservations
    )
}