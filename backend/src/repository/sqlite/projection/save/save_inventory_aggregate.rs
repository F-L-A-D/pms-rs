use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::inventory_aggregate::InventoryAggregate,
};

const QUERY: &str = include_str!("save_inventory_aggregate.sql");

pub async fn save_inventory_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &InventoryAggregate,
) -> AppResult<()> {
    sqlx::query(QUERY)
        .bind(aggregate.service_date.to_string())
        .bind(&aggregate.room_class)
        .bind(aggregate.total_rooms)
        .bind(aggregate.out_of_order_rooms)
        .bind(aggregate.reservable_rooms)
        .bind(aggregate.confirmed_reservations)
        .bind(aggregate.pending_reservations)
        .bind(aggregate.cancelled_reservations)
        .bind(aggregate.available_rooms)
        .bind(aggregate.available_rooms_including_pending)
        .bind(aggregate.projection_version)
        .bind(aggregate.updated_at)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
