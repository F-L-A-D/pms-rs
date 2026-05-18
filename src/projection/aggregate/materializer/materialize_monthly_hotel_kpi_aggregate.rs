use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        materializer::materialize_monthly_room_class_kpi_aggregates::materialize_monthly_room_class_kpi_aggregates,
        model::monthly_hotel_kpi_aggregate::MonthlyHotelKpiAggregate,
    },
};

pub async fn materialize_monthly_hotel_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<MonthlyHotelKpiAggregate> {
    let rows = materialize_monthly_room_class_kpi_aggregates(tx, year_month).await?;

    let total_room_nights = rows.iter().map(|row| row.total_room_nights).sum();
    let out_of_order_room_nights = rows.iter().map(|row| row.out_of_order_room_nights).sum();
    let reservable_room_nights = rows.iter().map(|row| row.reservable_room_nights).sum();
    let sold_room_nights = rows.iter().map(|row| row.sold_room_nights).sum();
    let occupied_room_nights = rows.iter().map(|row| row.occupied_room_nights).sum();
    let room_revenue = rows.iter().map(|row| row.room_revenue).sum();
    let food_and_beverage_revenue = rows.iter().map(|row| row.food_and_beverage_revenue).sum();
    let other_revenue = rows.iter().map(|row| row.other_revenue).sum();
    let tax_amount = rows.iter().map(|row| row.tax_amount).sum();
    let total_revenue = rows.iter().map(|row| row.total_revenue).sum();

    Ok(MonthlyHotelKpiAggregate {
        year_month: year_month.to_string(),
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
        occupancy_rate: if reservable_room_nights == 0 {
            rust_decimal::Decimal::ZERO
        } else {
            rust_decimal::Decimal::from(sold_room_nights)
                / rust_decimal::Decimal::from(reservable_room_nights)
        },
        adr: if sold_room_nights == 0 {
            rust_decimal::Decimal::ZERO
        } else {
            room_revenue / rust_decimal::Decimal::from(sold_room_nights)
        },
        revpar: if reservable_room_nights == 0 {
            rust_decimal::Decimal::ZERO
        } else {
            room_revenue / rust_decimal::Decimal::from(reservable_room_nights)
        },
        projection_version: 1,
        updated_at: chrono::Utc::now(),
    })
}
