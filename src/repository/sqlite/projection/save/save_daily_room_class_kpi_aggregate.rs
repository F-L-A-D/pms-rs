use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::daily_room_class_kpi_aggregate::DailyRoomClassKpiAggregate,
};

const QUERY: &str = include_str!("save_daily_room_class_kpi_aggregate.sql");

pub async fn save_daily_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &DailyRoomClassKpiAggregate,
) -> AppResult<()> {
    sqlx::query(QUERY)
        .bind(aggregate.service_date.to_string())
        .bind(&aggregate.room_class)
        .bind(aggregate.total_rooms)
        .bind(aggregate.out_of_order_rooms)
        .bind(aggregate.reservable_rooms)
        .bind(aggregate.sold_room_nights)
        .bind(aggregate.occupied_rooms)
        .bind(aggregate.room_revenue.to_string())
        .bind(aggregate.food_and_beverage_revenue.to_string())
        .bind(aggregate.other_revenue.to_string())
        .bind(aggregate.tax_amount.to_string())
        .bind(aggregate.total_revenue.to_string())
        .bind(aggregate.occupancy_rate.to_string())
        .bind(aggregate.adr.to_string())
        .bind(aggregate.revpar.to_string())
        .bind(aggregate.projection_version)
        .bind(aggregate.updated_at)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
