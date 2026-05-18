use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_booking::{
        ReservationDailyRevenueAllocation, ReservationRevenueCategory,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationDailyRevenueAllocationRepository;

impl SqliteReservationDailyRevenueAllocationRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        allocation: &ReservationDailyRevenueAllocation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO reservation_daily_revenue_allocations (
                reservation_id,
                service_date,
                package_code,
                revenue_category,
                amount
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(allocation.reservation_id.to_string())
        .bind(allocation.service_date.to_string())
        .bind(&allocation.package_code)
        .bind(allocation.revenue_category.to_snake())
        .bind(allocation.amount.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationDailyRevenueAllocation>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                service_date,
                package_code,
                revenue_category,
                amount
            FROM reservation_daily_revenue_allocations
            WHERE reservation_id = ?1
            ORDER BY service_date, package_code, revenue_category
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_allocation).collect()
    }

    pub async fn delete_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query("DELETE FROM reservation_daily_revenue_allocations WHERE reservation_id = ?1")
            .bind(reservation_id.to_string())
            .execute(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(())
    }

    fn row_to_allocation(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<ReservationDailyRevenueAllocation> {
        Ok(ReservationDailyRevenueAllocation {
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            service_date: row
                .get::<String, _>("service_date")
                .parse::<NaiveDate>()
                .map_err(infra)?,
            package_code: row.get("package_code"),
            revenue_category: ReservationRevenueCategory::from_snake(
                row.get::<String, _>("revenue_category").as_str(),
            )
            .ok_or_else(|| infra("invalid reservation revenue category"))?,
            amount: row
                .get::<String, _>("amount")
                .parse::<Decimal>()
                .map_err(infra)?,
        })
    }
}
