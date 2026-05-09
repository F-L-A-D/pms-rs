use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::projection::
    hotel_inventory_projection_repository::
        SqliteHotelInventoryProjectionRepository;

pub async fn refresh_hotel_inventory_projection(
    tx: &mut Transaction<'_, Sqlite>,
    date: &str,
) -> AppResult<()> {

    let row =
        sqlx::query(
            r#"
            SELECT
                COALESCE(
                    SUM(reserved_rooms),
                    0
                ) as total
            FROM inventory_projection
            WHERE date = ?1
            "#
        )
        .bind(date)
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    let total: i32 =
        row.get("total");

    SqliteHotelInventoryProjectionRepository
        ::upsert(
            tx,
            date,
            total,
        )
        .await
        .map_err(
            AppError::Infrastructure
        )?;

    Ok(())
}