use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    domain::semantic::reservation_booking::ReservationRevenueCategory,
    error::app_error::{infra, AppResult},
    projection::aggregate::model::daily_room_class_kpi_aggregate_row::DailyRoomClassKpiAggregateRow,
};

const COUNT_QUERY: &str = include_str!("aggregate_daily_room_class_kpi_counts.sql");
const REVENUE_QUERY: &str = include_str!("aggregate_daily_room_class_kpi_revenue.sql");

pub async fn aggregate_daily_room_class_kpi(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<DailyRoomClassKpiAggregateRow>> {
    let count_rows = sqlx::query(COUNT_QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    let mut rows = Vec::new();

    for count_row in count_rows {
        let room_class: String = count_row.get("room_class");
        let revenue = aggregate_revenue(tx, service_date, &room_class).await?;

        rows.push(DailyRoomClassKpiAggregateRow {
            room_class,
            total_rooms: count_row.get("total_rooms"),
            out_of_order_rooms: count_row.get("out_of_order_rooms"),
            sold_room_nights: count_row.get("sold_room_nights"),
            occupied_rooms: count_row.get("occupied_rooms"),
            room_revenue: revenue.room,
            food_and_beverage_revenue: revenue.food_and_beverage,
            other_revenue: revenue.other,
            tax_amount: revenue.tax,
        });
    }

    Ok(rows)
}

#[derive(Debug)]
struct RevenueBreakdown {
    room: Decimal,
    food_and_beverage: Decimal,
    other: Decimal,
    tax: Decimal,
}

async fn aggregate_revenue(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
    room_class: &str,
) -> AppResult<RevenueBreakdown> {
    let revenue_rows = sqlx::query(REVENUE_QUERY)
        .bind(service_date.to_string())
        .bind(room_class)
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    let mut breakdown = RevenueBreakdown {
        room: Decimal::ZERO,
        food_and_beverage: Decimal::ZERO,
        other: Decimal::ZERO,
        tax: Decimal::ZERO,
    };

    for row in revenue_rows {
        let amount = row
            .get::<String, _>("amount")
            .parse::<Decimal>()
            .map_err(infra)?;

        let category = ReservationRevenueCategory::from_snake(
            row.get::<String, _>("revenue_category").as_str(),
        )
        .ok_or_else(|| infra("invalid reservation revenue category"))?;

        match category {
            ReservationRevenueCategory::Room => {
                breakdown.room += amount;
            }
            ReservationRevenueCategory::FoodAndBeverage => {
                breakdown.food_and_beverage += amount;
            }
            ReservationRevenueCategory::Other => {
                breakdown.other += amount;
            }
            ReservationRevenueCategory::Tax => {
                breakdown.tax += amount;
            }
        }
    }

    Ok(breakdown)
}
