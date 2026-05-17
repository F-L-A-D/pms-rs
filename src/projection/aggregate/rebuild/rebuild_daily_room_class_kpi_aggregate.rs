use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_daily_room_class_kpi_aggregate::refresh_daily_room_class_kpi_aggregate,
    },
};

pub async fn rebuild_daily_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    let service_dates = list_inventory_dates(tx).await?;

    for service_date in service_dates {
        refresh_daily_room_class_kpi_aggregate(tx, service_date).await?;
    }

    Ok(())
}
