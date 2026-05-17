use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::housekeeping_daily_workload_aggregate::HousekeepingDailyWorkloadAggregate,
};

const QUERY: &str = include_str!("save_housekeeping_daily_workload_aggregate.sql");

pub async fn save_housekeeping_daily_workload_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &HousekeepingDailyWorkloadAggregate,
) -> AppResult<()> {
    sqlx::query(QUERY)
        .bind(aggregate.service_date.to_string())
        .bind(&aggregate.room_class)
        .bind(aggregate.total_tracked_rooms)
        .bind(aggregate.dirty_rooms)
        .bind(aggregate.cleaning_rooms)
        .bind(aggregate.cleaned_rooms)
        .bind(aggregate.inspected_rooms)
        .bind(aggregate.occupied_rooms)
        .bind(aggregate.vacant_rooms)
        .bind(aggregate.out_of_order_rooms)
        .bind(aggregate.projection_version)
        .bind(aggregate.updated_at)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
