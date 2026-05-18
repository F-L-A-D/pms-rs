use std::collections::HashSet;

use chrono::Datelike;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_monthly_room_class_kpi_aggregate::refresh_monthly_room_class_kpi_aggregate,
    },
};

pub async fn rebuild_monthly_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    for year_month in list_inventory_months(tx).await? {
        refresh_monthly_room_class_kpi_aggregate(tx, &year_month).await?;
    }

    Ok(())
}

async fn list_inventory_months(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<String>> {
    let mut months = HashSet::new();

    for service_date in list_inventory_dates(tx).await? {
        months.insert(format!(
            "{:04}-{:02}",
            service_date.year(),
            service_date.month()
        ));
    }

    let mut months = months.into_iter().collect::<Vec<_>>();
    months.sort();

    Ok(months)
}
