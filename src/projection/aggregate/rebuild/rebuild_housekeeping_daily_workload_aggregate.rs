use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_housekeeping_daily_workload_aggregate::refresh_housekeeping_daily_workload_aggregate,
    },
};

pub async fn rebuild_housekeeping_daily_workload_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    let service_dates = list_inventory_dates(tx).await?;

    for service_date in service_dates {
        refresh_housekeeping_daily_workload_aggregate(tx, service_date).await?;
    }

    Ok(())
}
