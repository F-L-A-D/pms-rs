use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::deposit_application::DepositApplication,
    error::app_error::{infra, AppResult},
};

pub struct SqliteDepositApplicationRepository;

impl SqliteDepositApplicationRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        application: &DepositApplication,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO deposit_applications (
                id,
                deposit_id,
                receivable_id,
                amount,
                applied_at,
                reversed_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT(id) DO UPDATE SET
                deposit_id = excluded.deposit_id,
                receivable_id = excluded.receivable_id,
                amount = excluded.amount,
                applied_at = excluded.applied_at,
                reversed_at = excluded.reversed_at
            "#,
        )
        .bind(application.id.to_string())
        .bind(application.deposit_id.to_string())
        .bind(application.receivable_id.to_string())
        .bind(application.amount.to_string())
        .bind(application.applied_at.to_rfc3339())
        .bind(application.reversed_at.map(|value| value.to_rfc3339()))
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        application_id: Uuid,
    ) -> AppResult<Option<DepositApplication>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                deposit_id,
                receivable_id,
                amount,
                applied_at,
                reversed_at
            FROM deposit_applications
            WHERE id = ?1
            "#,
        )
        .bind(application_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(row_to_deposit_application).transpose()
    }
}

fn row_to_deposit_application(
    row: sqlx::sqlite::SqliteRow,
) -> AppResult<DepositApplication> {
    let id: String = row.try_get("id").map_err(infra)?;

    let deposit_id: String =
        row.try_get("deposit_id").map_err(infra)?;

    let receivable_id: String =
        row.try_get("receivable_id").map_err(infra)?;

    let amount: String =
        row.try_get("amount").map_err(infra)?;

    let applied_at: String =
        row.try_get("applied_at").map_err(infra)?;

    let reversed_at: Option<String> =
        row.try_get("reversed_at").map_err(infra)?;

    Ok(DepositApplication {
        id: Uuid::parse_str(&id).map_err(infra)?,
        deposit_id: Uuid::parse_str(&deposit_id).map_err(infra)?,
        receivable_id: Uuid::parse_str(&receivable_id).map_err(infra)?,
        amount: amount.parse::<Decimal>().map_err(infra)?,
        applied_at: DateTime::parse_from_rfc3339(&applied_at)
            .map_err(infra)?
            .with_timezone(&Utc),
        reversed_at: reversed_at
            .map(|value| {
                DateTime::parse_from_rfc3339(&value)
                    .map(|parsed| parsed.with_timezone(&Utc))
            })
            .transpose()
            .map_err(infra)?,
    })
}