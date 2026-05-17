use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::inventory_aggregate_row::InventoryAggregateRow,
};

const QUERY: &str = include_str!("aggregate_inventory_date.sql");

pub async fn aggregate_inventory_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<InventoryAggregateRow>> {
    let rows = sqlx::query(QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(rows
        .iter()
        .map(|row| InventoryAggregateRow {
            room_class: row.get("room_class"),
            total_rooms: row.get("total_rooms"),
            out_of_order_rooms: row.get("out_of_order_rooms"),
            confirmed_reservations: row.get("confirmed_reservations"),
            pending_reservations: row.get("pending_reservations"),
            cancelled_reservations: row.get("cancelled_reservations"),
        })
        .collect())
}
