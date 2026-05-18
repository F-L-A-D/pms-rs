use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use crate::{
    domain::semantic::{
        reservation_booking::ReservationRevenueCategory, revenue_summary::RevenueSummaryLine,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteRevenueSummaryRepository;

impl SqliteRevenueSummaryRepository {
    pub async fn summarize_daily(
        tx: &mut Transaction<'_, Sqlite>,
        service_date: NaiveDate,
    ) -> AppResult<Vec<RevenueSummaryLine>> {
        let rows = sqlx::query(
            r#"
            SELECT
                service_date,
                revenue_category,
                department_code,
                account_code,
                SUM(CAST(amount AS REAL)) AS amount
            FROM reservation_daily_revenue_allocations
            WHERE service_date = ?1
            GROUP BY service_date, revenue_category, department_code, account_code
            ORDER BY revenue_category, department_code, account_code
            "#,
        )
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_line_with_date).collect()
    }

    pub async fn summarize_monthly(
        tx: &mut Transaction<'_, Sqlite>,
        year_month: &str,
    ) -> AppResult<Vec<RevenueSummaryLine>> {
        let rows = sqlx::query(
            r#"
            SELECT
                revenue_category,
                department_code,
                account_code,
                SUM(CAST(amount AS REAL)) AS amount
            FROM reservation_daily_revenue_allocations
            WHERE substr(service_date, 1, 7) = ?1
            GROUP BY revenue_category, department_code, account_code
            ORDER BY revenue_category, department_code, account_code
            "#,
        )
        .bind(year_month)
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_line_without_date).collect()
    }

    fn row_to_line_with_date(row: &sqlx::sqlite::SqliteRow) -> AppResult<RevenueSummaryLine> {
        Ok(RevenueSummaryLine {
            service_date: Some(
                row.get::<String, _>("service_date")
                    .parse::<NaiveDate>()
                    .map_err(infra)?,
            ),
            revenue_category: Self::revenue_category(row)?,
            department_code: row.get("department_code"),
            account_code: row.get("account_code"),
            amount: Decimal::try_from(row.get::<f64, _>("amount")).map_err(infra)?,
        })
    }

    fn row_to_line_without_date(row: &sqlx::sqlite::SqliteRow) -> AppResult<RevenueSummaryLine> {
        Ok(RevenueSummaryLine {
            service_date: None,
            revenue_category: Self::revenue_category(row)?,
            department_code: row.get("department_code"),
            account_code: row.get("account_code"),
            amount: Decimal::try_from(row.get::<f64, _>("amount")).map_err(infra)?,
        })
    }

    fn revenue_category(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationRevenueCategory> {
        ReservationRevenueCategory::from_snake(row.get::<String, _>("revenue_category").as_str())
            .ok_or_else(|| infra("invalid reservation revenue category"))
    }
}
