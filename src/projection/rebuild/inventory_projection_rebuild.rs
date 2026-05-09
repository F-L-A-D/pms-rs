use sqlx::{
    Sqlite,
    Transaction,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::domain::reservation::
    ReservationStatus;

use crate::projection::service::
    inventory_projection_service::
        apply_reservation_projection;

use crate::repository::sqlite::{
    operational::reservation_repository::
        SqliteReservationRepository,

    projection::inventory_projection_repository::
        SqliteInventoryProjectionRepository,
};

pub async fn rebuild_inventory_projection(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    SqliteInventoryProjectionRepository
        ::delete_all(
            tx,
        )
        .await
        .map_err(
            AppError::Infrastructure
        )?;

    let reservations =
        SqliteReservationRepository
            ::find_all(
                tx,
            )
            .await
            .map_err(
                AppError::Infrastructure
            )?;

    for reservation in reservations {

        if reservation
            .reservation_status
            != ReservationStatus::Active
        {
            continue;
        }

        apply_reservation_projection(
            tx,
            &reservation,
        )
        .await?;
    }

    Ok(())
}