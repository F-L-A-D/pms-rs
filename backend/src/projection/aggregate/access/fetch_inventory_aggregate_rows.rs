use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::inventory_aggregate_row::InventoryAggregateRow,
    repository::sqlite::projection::aggregate::aggregate_inventory_date::aggregate_inventory_date,
};

pub async fn fetch_inventory_aggregate_rows(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<InventoryAggregateRow>> {
    aggregate_inventory_date(tx, service_date).await
}
