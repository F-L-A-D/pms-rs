use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::deposit_refund::DepositRefund,
    error::app_error::{infra, AppResult},
};

pub struct SqliteDepositRefundRepository;

impl SqliteDepositRefundRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, refund: &DepositRefund) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO deposit_refunds (
                id,
                deposit_id,
                amount,
                reason,
                refunded_at,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(refund.id.to_string())
        .bind(refund.deposit_id.to_string())
        .bind(refund.amount.to_string())
        .bind(refund.reason.as_deref())
        .bind(refund.refunded_at.to_rfc3339())
        .bind(refund.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<DepositRefund>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                deposit_id,
                amount,
                reason,
                refunded_at,
                created_at
            FROM deposit_refunds
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(Self::from_row).transpose()
    }

    fn from_row(row: sqlx::sqlite::SqliteRow) -> AppResult<DepositRefund> {
        let id = Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?;

        let deposit_id =
            Uuid::parse_str(row.get::<String, _>("deposit_id").as_str()).map_err(infra)?;

        let amount = row
            .get::<String, _>("amount")
            .parse::<Decimal>()
            .map_err(infra)?;

        let refunded_at =
            DateTime::parse_from_rfc3339(row.get::<String, _>("refunded_at").as_str())
                .map_err(infra)?
                .with_timezone(&Utc);

        let created_at = DateTime::parse_from_rfc3339(row.get::<String, _>("created_at").as_str())
            .map_err(infra)?
            .with_timezone(&Utc);

        DepositRefund::new(
            id,
            deposit_id,
            amount,
            row.get::<Option<String>, _>("reason"),
            refunded_at,
            created_at,
        )
        .map_err(infra)
    }
}
