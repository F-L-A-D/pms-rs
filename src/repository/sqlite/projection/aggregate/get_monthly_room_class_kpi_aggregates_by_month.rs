use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::aggregate::model::monthly_room_class_kpi_aggregate::MonthlyRoomClassKpiAggregate,
};

pub async fn get_monthly_room_class_kpi_aggregates_by_month(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<Vec<MonthlyRoomClassKpiAggregate>> {
    let rows = sqlx::query(
        r#"
        SELECT *
        FROM monthly_room_class_kpi_aggregates
        WHERE year_month = ?
        ORDER BY room_class
        "#,
    )
    .bind(year_month)
    .fetch_all(&mut **tx)
    .await
    .map_err(infra)?;

    rows.iter()
        .map(|row| {
            Ok(MonthlyRoomClassKpiAggregate {
                year_month: row.get("year_month"),
                room_class: row.get("room_class"),
                total_room_nights: row.get("total_room_nights"),
                out_of_order_room_nights: row.get("out_of_order_room_nights"),
                reservable_room_nights: row.get("reservable_room_nights"),
                sold_room_nights: row.get("sold_room_nights"),
                occupied_room_nights: row.get("occupied_room_nights"),
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
