use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::housekeeping_daily_workload_aggregate::HousekeepingDailyWorkloadAggregate,
};

const QUERY: &str = include_str!("get_housekeeping_daily_workload_aggregates_by_date.sql");

pub async fn get_housekeeping_daily_workload_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<HousekeepingDailyWorkloadAggregate>> {
    let rows = sqlx::query(QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(rows
        .iter()
        .map(|row| HousekeepingDailyWorkloadAggregate {
            service_date: row.get::<NaiveDate, _>("service_date"),
            room_class: row.get("room_class"),
            total_tracked_rooms: row.get("total_tracked_rooms"),
            dirty_rooms: row.get("dirty_rooms"),
            cleaning_rooms: row.get("cleaning_rooms"),
            cleaned_rooms: row.get("cleaned_rooms"),
            inspected_rooms: row.get("inspected_rooms"),
            occupied_rooms: row.get("occupied_rooms"),
            vacant_rooms: row.get("vacant_rooms"),
            out_of_order_rooms: row.get("out_of_order_rooms"),
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
        .collect())
}
