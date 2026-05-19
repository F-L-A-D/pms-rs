use chrono::{NaiveDate, Utc};

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        access::fetch_inventory_aggregate_rows::fetch_inventory_aggregate_rows,
        model::inventory_aggregate::InventoryAggregate,
    },
};

pub async fn materialize_inventory_aggregates(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<InventoryAggregate>> {
    let rows = fetch_inventory_aggregate_rows(tx, service_date).await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let reservable_rooms = row.total_rooms - row.out_of_order_rooms;
            let available_rooms = reservable_rooms - row.confirmed_reservations;
            let available_rooms_including_pending = available_rooms - row.pending_reservations;

            InventoryAggregate {
                service_date,
                room_class: row.room_class,
                total_rooms: row.total_rooms,
                out_of_order_rooms: row.out_of_order_rooms,
                reservable_rooms,
                confirmed_reservations: row.confirmed_reservations,
                pending_reservations: row.pending_reservations,
                cancelled_reservations: row.cancelled_reservations,
                available_rooms,
                available_rooms_including_pending,
                projection_version: 1,
                updated_at: Utc::now(),
            }
        })
        .collect())
}
