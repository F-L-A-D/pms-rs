use chrono::{NaiveDate, Utc};

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::{
        access::fetch_daily_room_class_kpi_aggregate_rows::fetch_daily_room_class_kpi_aggregate_rows,
        model::daily_room_class_kpi_aggregate::DailyRoomClassKpiAggregate,
    },
};

pub async fn materialize_daily_room_class_kpi_aggregates(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<DailyRoomClassKpiAggregate>> {
    let rows = fetch_daily_room_class_kpi_aggregate_rows(tx, service_date).await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let reservable_rooms = row.total_rooms - row.out_of_order_rooms;
            let total_revenue = row.room_revenue
                + row.food_and_beverage_revenue
                + row.other_revenue
                + row.tax_amount;
            let occupancy_rate = divide(row.sold_room_nights, reservable_rooms);
            let adr = divide_decimal(row.room_revenue, row.sold_room_nights);
            let revpar = divide_decimal(row.room_revenue, reservable_rooms);

            DailyRoomClassKpiAggregate {
                service_date,
                room_class: row.room_class,
                total_rooms: row.total_rooms,
                out_of_order_rooms: row.out_of_order_rooms,
                reservable_rooms,
                sold_room_nights: row.sold_room_nights,
                occupied_rooms: row.occupied_rooms,
                room_revenue: row.room_revenue,
                food_and_beverage_revenue: row.food_and_beverage_revenue,
                other_revenue: row.other_revenue,
                tax_amount: row.tax_amount,
                total_revenue,
                occupancy_rate,
                adr,
                revpar,
                projection_version: 1,
                updated_at: Utc::now(),
            }
        })
        .collect())
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
