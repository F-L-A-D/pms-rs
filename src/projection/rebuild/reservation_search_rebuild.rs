use sqlx::{
    Sqlite,
    Transaction,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::projection::materializer::
    reservation_search_materializer::
    materialize_reservation_search_projection;

use crate::repository::sqlite::operational::
    reservation_repository::
    SqliteReservationRepository;

use crate::repository::sqlite::projection::
    reservation_search_projection_repository::
    ReservationSearchProjectionRepository;

pub async fn rebuild_reservation_search_projection(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    ReservationSearchProjectionRepository
        ::delete_all(tx)
        .await?;

    let reservations =
        SqliteReservationRepository
            ::find_all(tx)
            .await
            .map_err(
                AppError::Infrastructure
            )?;

    for reservation in reservations {

        let projection =
            materialize_reservation_search_projection(
                tx,
                reservation.id,
            )
            .await?;

        ReservationSearchProjectionRepository
            ::upsert(
                tx,
                &projection,
            )
            .await?;
    }

    Ok(())
}