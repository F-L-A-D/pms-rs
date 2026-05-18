use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::error::app_error::{infra, AppResult};

pub async fn delete_daily_room_class_kpi_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    sqlx::query("DELETE FROM daily_room_class_kpi_aggregates WHERE service_date = ?")
        .bind(service_date.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
