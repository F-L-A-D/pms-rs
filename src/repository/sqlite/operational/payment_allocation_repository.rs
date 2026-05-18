use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::payment_allocation::PaymentAllocation,
    error::app_error::{infra, AppResult},
};

pub struct SqlitePaymentAllocationRepository;

impl SqlitePaymentAllocationRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        allocation: &PaymentAllocation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO payment_allocations (
                id,
                payment_id,
                receivable_id,
                amount,
                allocated_at,
                reversed_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(allocation.id.to_string())
        .bind(allocation.payment_id.to_string())
        .bind(allocation.receivable_id.to_string())
        .bind(allocation.amount.to_string())
        .bind(allocation.allocated_at.to_rfc3339())
        .bind(allocation.reversed_at.map(|value| value.to_rfc3339()))
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_receivable_id(
        tx: &mut Transaction<'_, Sqlite>,
        receivable_id: Uuid,
    ) -> AppResult<Vec<PaymentAllocation>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                payment_id,
                receivable_id,
                amount,
                allocated_at,
                reversed_at
            FROM payment_allocations
            WHERE receivable_id = ?1
            ORDER BY allocated_at, id
            "#,
        )
        .bind(receivable_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_allocation).collect()
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<PaymentAllocation>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                payment_id,
                receivable_id,
                amount,
                allocated_at,
                reversed_at
            FROM payment_allocations
            WHERE id = ?1
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_allocation(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn mark_reversed(
        tx: &mut Transaction<'_, Sqlite>,
        allocation: &PaymentAllocation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE payment_allocations
            SET reversed_at = ?2
            WHERE id = ?1
            "#,
        )
        .bind(allocation.id.to_string())
        .bind(allocation.reversed_at.map(|value| value.to_rfc3339()))
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    fn row_to_allocation(row: &sqlx::sqlite::SqliteRow) -> AppResult<PaymentAllocation> {
        Ok(PaymentAllocation {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            payment_id: Uuid::parse_str(row.get::<String, _>("payment_id").as_str())
                .map_err(infra)?,
            receivable_id: Uuid::parse_str(row.get::<String, _>("receivable_id").as_str())
                .map_err(infra)?,
            amount: row.get::<String, _>("amount").parse().map_err(infra)?,
            allocated_at: row
                .get::<String, _>("allocated_at")
                .parse::<DateTime<Utc>>()
                .map_err(infra)?,
            reversed_at: row
                .get::<Option<String>, _>("reversed_at")
                .map(|value| value.parse::<DateTime<Utc>>())
                .transpose()
                .map_err(infra)?,
        })
    }
}
