use chrono::{NaiveDate, Utc};

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        access::fetch_daily_room_class_kpi_aggregate_rows::fetch_daily_room_class_kpi_aggregate_rows,
        model::daily_hotel_kpi_aggregate::DailyHotelKpiAggregate,
    },
};

pub async fn materialize_daily_hotel_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<DailyHotelKpiAggregate> {
    let rows = fetch_daily_room_class_kpi_aggregate_rows(tx, service_date).await?;

    let total_rooms = rows.iter().map(|row| row.total_rooms).sum();
    let out_of_order_rooms = rows.iter().map(|row| row.out_of_order_rooms).sum();
    let reservable_rooms = total_rooms - out_of_order_rooms;
    let sold_room_nights = rows.iter().map(|row| row.sold_room_nights).sum();
    let occupied_rooms = rows.iter().map(|row| row.occupied_rooms).sum();
    let room_revenue = rows.iter().map(|row| row.room_revenue).sum();
    let food_and_beverage_revenue = rows.iter().map(|row| row.food_and_beverage_revenue).sum();
    let other_revenue = rows.iter().map(|row| row.other_revenue).sum();
    let tax_amount = rows.iter().map(|row| row.tax_amount).sum();
    let total_revenue = room_revenue + food_and_beverage_revenue + other_revenue + tax_amount;

    Ok(DailyHotelKpiAggregate {
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
        occupancy_rate: divide(sold_room_nights, reservable_rooms),
        adr: divide_decimal(room_revenue, sold_room_nights),
        revpar: divide_decimal(room_revenue, reservable_rooms),
        projection_version: 1,
        updated_at: Utc::now(),
    })
}

fn divide(numerator: i64, denominator: i64) -> Decimal {
    if denominator == 0 {
        Decimal::ZERO
    } else {
        Decimal::from(numerator) / Decimal::from(denominator)
    }
}

fn divide_decimal(numerator: Decimal, denominator: i64) -> Decimal {
    if denominator == 0 {
        Decimal::ZERO
    } else {
        numerator / Decimal::from(denominator)
    }
}
