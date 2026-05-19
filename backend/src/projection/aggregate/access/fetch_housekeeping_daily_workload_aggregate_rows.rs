use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::housekeeping_daily_workload_aggregate_row::HousekeepingDailyWorkloadAggregateRow,
    repository::sqlite::projection::aggregate::aggregate_housekeeping_daily_workload::aggregate_housekeeping_daily_workload,
};

pub async fn fetch_housekeeping_daily_workload_aggregate_rows(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<HousekeepingDailyWorkloadAggregateRow>> {
    aggregate_housekeeping_daily_workload(tx, service_date).await
}
