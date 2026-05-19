use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::daily_hotel_kpi_aggregate::DailyHotelKpiAggregate,
};

pub async fn save_daily_hotel_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &DailyHotelKpiAggregate,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO daily_hotel_kpi_aggregates (
            service_date,
            total_rooms,
            out_of_order_rooms,
            reservable_rooms,
            sold_room_nights,
            occupied_rooms,
            room_revenue,
            food_and_beverage_revenue,
            other_revenue,
            tax_amount,
            total_revenue,
            occupancy_rate,
            adr,
            revpar,
            projection_version,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(service_date)
        DO UPDATE SET
            total_rooms                 = excluded.total_rooms,
            out_of_order_rooms          = excluded.out_of_order_rooms,
            reservable_rooms            = excluded.reservable_rooms,
            sold_room_nights            = excluded.sold_room_nights,
            occupied_rooms              = excluded.occupied_rooms,
            room_revenue                = excluded.room_revenue,
            food_and_beverage_revenue   = excluded.food_and_beverage_revenue,
            other_revenue               = excluded.other_revenue,
            tax_amount                  = excluded.tax_amount,
            total_revenue               = excluded.total_revenue,
            occupancy_rate              = excluded.occupancy_rate,
            adr                         = excluded.adr,
            revpar                      = excluded.revpar,
            projection_version          = excluded.projection_version,
            updated_at                  = excluded.updated_at
        "#,
    )
    .bind(aggregate.service_date.to_string())
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
