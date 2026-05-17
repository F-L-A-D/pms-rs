use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::housekeeping_daily_workload_aggregate::HousekeepingDailyWorkloadAggregate,
    repository::sqlite::projection::aggregate::get_housekeeping_daily_workload_aggregates_by_date::get_housekeeping_daily_workload_aggregates_by_date,
};

pub async fn fetch_housekeeping_daily_workload_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<HousekeepingDailyWorkloadAggregate>> {
    get_housekeeping_daily_workload_aggregates_by_date(tx, service_date).await
}
