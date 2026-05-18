use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_inventory_aggregate::refresh_inventory_aggregate,
    },
};

pub async fn rebuild_inventory_aggregate(tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    let service_dates = list_inventory_dates(tx).await?;

    for service_date in service_dates {
        refresh_inventory_aggregate(tx, service_date).await?;
    }

    Ok(())
}
