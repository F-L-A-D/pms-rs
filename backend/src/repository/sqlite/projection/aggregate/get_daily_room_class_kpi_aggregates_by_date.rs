use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::daily_room_class_kpi_aggregate::DailyRoomClassKpiAggregate,
};

const QUERY: &str = include_str!("get_daily_room_class_kpi_aggregates_by_date.sql");

pub async fn get_daily_room_class_kpi_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<DailyRoomClassKpiAggregate>> {
    let rows = sqlx::query(QUERY)
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

    rows.iter()
        .map(|row| {
            Ok(DailyRoomClassKpiAggregate {
                service_date: row.get::<NaiveDate, _>("service_date"),
                room_class: row.get("room_class"),
                total_rooms: row.get("total_rooms"),
                out_of_order_rooms: row.get("out_of_order_rooms"),
                reservable_rooms: row.get("reservable_rooms"),
                sold_room_nights: row.get("sold_room_nights"),
                occupied_rooms: row.get("occupied_rooms"),
                room_revenue: parse_decimal(row, "room_revenue")?,
                food_and_beverage_revenue: parse_decimal(row, "food_and_beverage_revenue")?,
                other_revenue: parse_decimal(row, "other_revenue")?,
                tax_amount: parse_decimal(row, "tax_amount")?,
                total_revenue: parse_decimal(row, "total_revenue")?,
                occupancy_rate: parse_decimal(row, "occupancy_rate")?,
                adr: parse_decimal(row, "adr")?,
                revpar: parse_decimal(row, "revpar")?,
                projection_version: row.get("projection_version"),
                updated_at: row.get("updated_at"),
            })
        })
        .collect()
}

fn parse_decimal(row: &sqlx::sqlite::SqliteRow, column: &str) -> AppResult<Decimal> {
    row.get::<String, _>(column)
        .parse::<Decimal>()
        .map_err(infra)
}
