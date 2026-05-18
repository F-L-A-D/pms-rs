use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::inventory_aggregate::InventoryAggregate,
    repository::sqlite::projection::aggregate::get_inventory_aggregates_by_date::get_inventory_aggregates_by_date,
};

pub async fn fetch_inventory_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<InventoryAggregate>> {
    get_inventory_aggregates_by_date(tx, service_date).await
}
