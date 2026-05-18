use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_booking::{
        ReservationPackageBreakdown, ReservationRevenueCategory,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationPackageBreakdownRepository;

impl SqliteReservationPackageBreakdownRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        breakdown: &ReservationPackageBreakdown,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_package_breakdowns (
                reservation_id,
                package_code,
                revenue_category,
                amount
            )
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(breakdown.reservation_id.to_string())
        .bind(&breakdown.package_code)
        .bind(breakdown.revenue_category.to_snake())
        .bind(breakdown.amount.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationPackageBreakdown>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                package_code,
                revenue_category,
                amount
            FROM reservation_package_breakdowns
            WHERE reservation_id = ?1
            ORDER BY package_code, revenue_category
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_breakdown).collect()
    }

    pub async fn delete_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query("DELETE FROM reservation_package_breakdowns WHERE reservation_id = ?1")
            .bind(reservation_id.to_string())
            .execute(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(())
    }

    fn row_to_breakdown(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationPackageBreakdown> {
        let revenue_category = ReservationRevenueCategory::from_snake(
            row.get::<String, _>("revenue_category").as_str(),
        )
        .ok_or_else(|| infra("invalid reservation revenue category"))?;

        Ok(ReservationPackageBreakdown {
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            package_code: row.get("package_code"),
            revenue_category,
            amount: row.get::<String, _>("amount").parse().map_err(infra)?,
        })
    }
}
