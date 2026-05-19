use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::monthly_room_class_kpi_aggregate::MonthlyRoomClassKpiAggregate,
};

pub async fn save_monthly_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &MonthlyRoomClassKpiAggregate,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO monthly_room_class_kpi_aggregates (
            year_month,
            room_class,
            total_room_nights,
            out_of_order_room_nights,
            reservable_room_nights,
            sold_room_nights,
            occupied_room_nights,
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
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(year_month, room_class)
        DO UPDATE SET
            total_room_nights           = excluded.total_room_nights,
            out_of_order_room_nights    = excluded.out_of_order_room_nights,
            reservable_room_nights      = excluded.reservable_room_nights,
            sold_room_nights            = excluded.sold_room_nights,
            occupied_room_nights        = excluded.occupied_room_nights,
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
    .bind(&aggregate.year_month)
    .bind(&aggregate.room_class)
    .bind(aggregate.total_room_nights)
    .bind(aggregate.out_of_order_room_nights)
    .bind(aggregate.reservable_room_nights)
    .bind(aggregate.sold_room_nights)
    .bind(aggregate.occupied_room_nights)
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
