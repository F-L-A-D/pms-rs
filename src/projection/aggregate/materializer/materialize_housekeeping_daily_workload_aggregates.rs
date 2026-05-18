use chrono::{NaiveDate, Utc};

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        access::fetch_housekeeping_daily_workload_aggregate_rows::fetch_housekeeping_daily_workload_aggregate_rows,
        model::housekeeping_daily_workload_aggregate::HousekeepingDailyWorkloadAggregate,
    },
};

pub async fn materialize_housekeeping_daily_workload_aggregates(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<HousekeepingDailyWorkloadAggregate>> {
    let rows = fetch_housekeeping_daily_workload_aggregate_rows(tx, service_date).await?;

    Ok(rows
        .into_iter()
        .map(|row| HousekeepingDailyWorkloadAggregate {
            service_date,
            room_class: row.room_class,
            total_tracked_rooms: row.total_tracked_rooms,
            dirty_rooms: row.dirty_rooms,
            cleaning_rooms: row.cleaning_rooms,
            cleaned_rooms: row.cleaned_rooms,
            inspected_rooms: row.inspected_rooms,
            occupied_rooms: row.occupied_rooms,
            vacant_rooms: row.vacant_rooms,
            out_of_order_rooms: row.out_of_order_rooms,
            projection_version: 1,
            updated_at: Utc::now(),
        })
        .collect())
}
