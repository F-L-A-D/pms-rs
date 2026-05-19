use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::inventory_aggregate::InventoryAggregate,
};

const QUERY: &str = include_str!("get_inventory_aggregates_by_date.sql");

pub async fn get_inventory_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<InventoryAggregate>> {
    let rows = sqlx::query(QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(rows
        .iter()
        .map(|row| InventoryAggregate {
            service_date: row.get::<NaiveDate, _>("service_date"),
            room_class: row.get("room_class"),
            total_rooms: row.get("total_rooms"),
            out_of_order_rooms: row.get("out_of_order_rooms"),
            reservable_rooms: row.get("reservable_rooms"),
            confirmed_reservations: row.get("confirmed_reservations"),
            pending_reservations: row.get("pending_reservations"),
            cancelled_reservations: row.get("cancelled_reservations"),
            available_rooms: row.get("available_rooms"),
            available_rooms_including_pending: row.get("available_rooms_including_pending"),
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
        .collect())
}
