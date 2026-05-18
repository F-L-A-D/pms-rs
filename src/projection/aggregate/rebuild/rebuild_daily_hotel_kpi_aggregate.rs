use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_daily_hotel_kpi_aggregate::refresh_daily_hotel_kpi_aggregate,
    },
};

pub async fn rebuild_daily_hotel_kpi_aggregate(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    for service_date in list_inventory_dates(tx).await? {
        refresh_daily_hotel_kpi_aggregate(tx, service_date).await?;
    }

    Ok(())
}
