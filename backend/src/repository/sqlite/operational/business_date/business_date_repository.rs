use chrono::{DateTime, NaiveDate, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::business_date::{BusinessDate, BusinessDateStatus},
    error::app_error::{infra, AppResult},
};

pub struct SqliteBusinessDateRepository;

impl SqliteBusinessDateRepository {
    pub async fn insert(
        tx: &mut Transaction<'_, Sqlite>,
        business_date: &BusinessDate,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO business_dates (
                id,
                business_date,
                status,
                opened_at,
                closing_started_at,
                closed_at,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
        )
        .bind(business_date.id.to_string())
        .bind(business_date.business_date.to_string())
        .bind(business_date.status.to_snake())
        .bind(business_date.opened_at.to_rfc3339())
        .bind(
            business_date
                .closing_started_at
                .map(|value| value.to_rfc3339()),
        )
        .bind(business_date.closed_at.map(|value| value.to_rfc3339()))
        .bind(business_date.created_at.to_rfc3339())
        .bind(business_date.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn update(
        tx: &mut Transaction<'_, Sqlite>,
        business_date: &BusinessDate,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE business_dates
            SET
                status = ?2,
                opened_at = ?3,
                closing_started_at = ?4,
                closed_at = ?5,
                updated_at = ?6
            WHERE id = ?1
            "#,
        )
        .bind(business_date.id.to_string())
        .bind(business_date.status.to_snake())
        .bind(business_date.opened_at.to_rfc3339())
        .bind(
            business_date
                .closing_started_at
                .map(|value| value.to_rfc3339()),
        )
        .bind(business_date.closed_at.map(|value| value.to_rfc3339()))
        .bind(business_date.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_current_active(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Option<BusinessDate>> {
        Self::find_by_statuses(tx, &[BusinessDateStatus::Open, BusinessDateStatus::Closing]).await
    }

    pub async fn find_current_open(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Option<BusinessDate>> {
        Self::find_by_statuses(tx, &[BusinessDateStatus::Open]).await
    }

    pub async fn find_current_closing(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Option<BusinessDate>> {
        Self::find_by_statuses(tx, &[BusinessDateStatus::Closing]).await
    }

    async fn find_by_statuses(
        tx: &mut Transaction<'_, Sqlite>,
        statuses: &[BusinessDateStatus],
    ) -> AppResult<Option<BusinessDate>> {
        let status_values = statuses
            .iter()
            .map(BusinessDateStatus::to_snake)
            .collect::<Vec<_>>();

        let row = match status_values.as_slice() {
            ["open"] => sqlx::query(
                r#"
                        SELECT
                            id,
                            business_date,
                            status,
                            opened_at,
                            closing_started_at,
                            closed_at,
                            created_at,
                            updated_at
                        FROM business_dates
                        WHERE status = 'open'
                        ORDER BY business_date DESC
                        LIMIT 1
                        "#,
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?,
            ["closing"] => sqlx::query(
                r#"
                        SELECT
                            id,
                            business_date,
                            status,
                            opened_at,
                            closing_started_at,
                            closed_at,
                            created_at,
                            updated_at
                        FROM business_dates
                        WHERE status = 'closing'
                        ORDER BY business_date DESC
                        LIMIT 1
                        "#,
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?,
            _ => sqlx::query(
                r#"
                        SELECT
                            id,
                            business_date,
                            status,
                            opened_at,
                            closing_started_at,
                            closed_at,
                            created_at,
                            updated_at
                        FROM business_dates
                        WHERE status IN ('open', 'closing')
                        ORDER BY business_date DESC
                        LIMIT 1
                        "#,
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?,
        };

        match row {
            Some(row) => Ok(Some(Self::row_to_business_date(&row)?)),
            None => Ok(None),
        }
    }

    fn row_to_business_date(row: &sqlx::sqlite::SqliteRow) -> AppResult<BusinessDate> {
        let status = BusinessDateStatus::from_snake(row.get::<String, _>("status").as_str())
            .ok_or_else(|| infra("invalid business date status"))?;

        let closing_started_at =
            Self::parse_optional_datetime(row.get::<Option<String>, _>("closing_started_at"))?;

        let closed_at = Self::parse_optional_datetime(row.get::<Option<String>, _>("closed_at"))?;

        Ok(BusinessDate {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            business_date: NaiveDate::parse_from_str(
                row.get::<String, _>("business_date").as_str(),
                "%Y-%m-%d",
            )
            .map_err(infra)?,
            status,
            opened_at: DateTime::parse_from_rfc3339(row.get::<String, _>("opened_at").as_str())
                .map_err(infra)?
                .with_timezone(&Utc),
            closing_started_at,
            closed_at,
            created_at: DateTime::parse_from_rfc3339(row.get::<String, _>("created_at").as_str())
                .map_err(infra)?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(row.get::<String, _>("updated_at").as_str())
                .map_err(infra)?
                .with_timezone(&Utc),
        })
    }

    fn parse_optional_datetime(value: Option<String>) -> AppResult<Option<DateTime<Utc>>> {
        match value {
            Some(value) => Ok(Some(
                DateTime::parse_from_rfc3339(value.as_str())
                    .map_err(infra)?
                    .with_timezone(&Utc),
            )),
            None => Ok(None),
        }
    }
}
