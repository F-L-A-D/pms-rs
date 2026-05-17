use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::housekeeping_daily_workload_aggregate_row::HousekeepingDailyWorkloadAggregateRow,
};

const QUERY: &str = include_str!("aggregate_housekeeping_daily_workload.sql");

pub async fn aggregate_housekeeping_daily_workload(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<HousekeepingDailyWorkloadAggregateRow>> {
    let rows = sqlx::query(QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(rows
        .iter()
        .map(|row| HousekeepingDailyWorkloadAggregateRow {
            room_class: row.get("room_class"),
            total_tracked_rooms: row.get("total_tracked_rooms"),
            dirty_rooms: row.get("dirty_rooms"),
            cleaning_rooms: row.get("cleaning_rooms"),
            cleaned_rooms: row.get("cleaned_rooms"),
            inspected_rooms: row.get("inspected_rooms"),
            occupied_rooms: row.get("occupied_rooms"),
            vacant_rooms: row.get("vacant_rooms"),
            out_of_order_rooms: row.get("out_of_order_rooms"),
        })
        .collect())
}
