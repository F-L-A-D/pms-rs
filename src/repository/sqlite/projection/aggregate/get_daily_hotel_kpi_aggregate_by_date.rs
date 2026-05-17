use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::daily_hotel_kpi_aggregate::DailyHotelKpiAggregate,
};

pub async fn get_daily_hotel_kpi_aggregate_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Option<DailyHotelKpiAggregate>> {
    let row = sqlx::query(
        r#"
        SELECT *
        FROM daily_hotel_kpi_aggregates
        WHERE service_date = ?
        "#,
    )
    .bind(service_date.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(infra)?;

    row.map(|row| {
        Ok(DailyHotelKpiAggregate {
            service_date: row.get::<NaiveDate, _>("service_date"),
            total_rooms: row.get("total_rooms"),
            out_of_order_rooms: row.get("out_of_order_rooms"),
            reservable_rooms: row.get("reservable_rooms"),
            sold_room_nights: row.get("sold_room_nights"),
            occupied_rooms: row.get("occupied_rooms"),
            room_revenue: parse_decimal(&row, "room_revenue")?,
            food_and_beverage_revenue: parse_decimal(&row, "food_and_beverage_revenue")?,
            other_revenue: parse_decimal(&row, "other_revenue")?,
            tax_amount: parse_decimal(&row, "tax_amount")?,
            total_revenue: parse_decimal(&row, "total_revenue")?,
            occupancy_rate: parse_decimal(&row, "occupancy_rate")?,
            adr: parse_decimal(&row, "adr")?,
            revpar: parse_decimal(&row, "revpar")?,
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
    })
    .transpose()
}

fn parse_decimal(row: &sqlx::sqlite::SqliteRow, column: &str) -> AppResult<Decimal> {
    row.get::<String, _>(column)
        .parse::<Decimal>()
        .map_err(infra)
}
