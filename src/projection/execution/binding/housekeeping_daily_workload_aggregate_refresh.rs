use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_housekeeping_daily_workload_aggregate::refresh_housekeeping_daily_workload_aggregate,
        invalidation::projection_invalidation::ProjectionRefreshTarget,
    },
};

pub async fn execute_housekeeping_daily_workload_aggregate_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    match target {
        ProjectionRefreshTarget::RoomDate { date } => {
            let service_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(infra)?;

            refresh_housekeeping_daily_workload_aggregate(tx, service_date).await?;
        }

        ProjectionRefreshTarget::Global => {
            for service_date in list_inventory_dates(tx).await? {
                refresh_housekeeping_daily_workload_aggregate(tx, service_date).await?;
            }
        }

        _ => {}
    }

    Ok(())
}
