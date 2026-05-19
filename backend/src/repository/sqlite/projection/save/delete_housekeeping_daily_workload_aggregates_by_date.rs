use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::error::app_error::{infra, AppResult};

pub async fn delete_housekeeping_daily_workload_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    sqlx::query(
        r#"
        DELETE FROM housekeeping_daily_workload_aggregates
        WHERE service_date = ?1
        "#,
    )
    .bind(service_date.to_string())
    .execute(&mut **tx)
    .await
    .map_err(infra)?;

    Ok(())
}
