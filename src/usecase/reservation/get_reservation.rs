use crate::db::connection::Db;

use crate::domain::reservation::Reservation;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::
    reservation_repository::
        SqliteReservationRepository;

pub async fn get_reservation(
    db: &Db,
    reservation_id: &str,
) -> AppResult<Option<Reservation>> {

    let mut tx =
        db.begin_tx().await;

    let result =
        SqliteReservationRepository
            ::find_by_id(
                &mut tx,
                reservation_id,
            )
            .await
            .map_err(
                AppError::Infrastructure
            );

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    result
}