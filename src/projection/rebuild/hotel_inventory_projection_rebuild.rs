use sqlx::{
    Sqlite,
    Transaction,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::projection::materializer::
    inventory_materializer::
        materialize_hotel_inventory;

use crate::repository::sqlite::projection::{
    hotel_inventory_projection_repository::
        SqliteHotelInventoryProjectionRepository,

    inventory_projection_repository::
        SqliteInventoryProjectionRepository,
};

pub async fn rebuild_hotel_inventory_projection(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()>
{
    SqliteHotelInventoryProjectionRepository
        ::delete_all(tx)
        .await
        .map_err(
            AppError::Infrastructure
        )?;

    let inventory =
        SqliteInventoryProjectionRepository
            ::list_all(tx)
            .await
            .map_err(
                AppError::Infrastructure
            )?;

    let projections =
        materialize_hotel_inventory(
            inventory,
        );

    for projection in projections {

        SqliteHotelInventoryProjectionRepository
            ::upsert(
                tx,
                &projection.date,
                projection.reserved_rooms,
            )
            .await
            .map_err(
                AppError::Infrastructure
            )?;
    }

    Ok(())
}