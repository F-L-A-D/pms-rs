use std::collections::HashSet;

use chrono::Datelike;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        access::list_inventory_dates::list_inventory_dates,
        aggregate::refresh::refresh_monthly_hotel_kpi_aggregate::refresh_monthly_hotel_kpi_aggregate,
        invalidation::projection_invalidation::ProjectionRefreshTarget,
    },
};

pub async fn execute_monthly_hotel_kpi_aggregate_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    match target {
        ProjectionRefreshTarget::KpiMonth { year_month } => {
            refresh_monthly_hotel_kpi_aggregate(tx, year_month).await?;
        }

        ProjectionRefreshTarget::Global => {
            for year_month in list_inventory_months(tx).await? {
                refresh_monthly_hotel_kpi_aggregate(tx, &year_month).await?;
            }
        }

        _ => {}
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
