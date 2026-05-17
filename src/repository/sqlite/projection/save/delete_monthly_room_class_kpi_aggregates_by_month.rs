use sqlx::{Sqlite, Transaction};

use crate::error::app_error::{infra, AppResult};

pub async fn delete_monthly_room_class_kpi_aggregates_by_month(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<()> {
    sqlx::query("DELETE FROM monthly_room_class_kpi_aggregates WHERE year_month = ?")
        .bind(year_month)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
