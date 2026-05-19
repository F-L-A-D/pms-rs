use sqlx::{Row, Sqlite, Transaction};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    domain::entity::receivable::{Receivable, ReceivableStatus},
    error::app_error::{infra, AppResult},
};

pub struct SqliteReceivableRepository;

impl SqliteReceivableRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, receivable: &Receivable) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO receivables (
                id,
                invoice_id,
                outstanding_amount,
                due_date,
                status
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(receivable.id.to_string())
        .bind(receivable.invoice_id.to_string())
        .bind(receivable.outstanding_amount.to_string())
        .bind(receivable.due_date.to_string())
        .bind(receivable.status.to_snake())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Receivable>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    invoice_id,
                    outstanding_amount,
                    due_date,
                    status
                FROM receivables
                WHERE id = ?1
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_receivable(&row)?)),

            None => Ok(None),
        }
    }

    pub async fn find_by_invoice_id(
        tx: &mut Transaction<'_, Sqlite>,
        invoice_id: Uuid,
    ) -> AppResult<Option<Receivable>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    invoice_id,
                    outstanding_amount,
                    due_date,
                    status
                FROM receivables
                WHERE invoice_id = ?1
                "#,
        )
        .bind(invoice_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_receivable(&row)?)),

            None => Ok(None),
        }
    }

    pub async fn list_all(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Receivable>> {
        let rows = sqlx::query(
            r#"
                SELECT
                    id,
                    invoice_id,
                    outstanding_amount,
                    due_date,
                    status
                FROM receivables
                ORDER BY due_date, id
                "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_receivable).collect()
    }

    fn row_to_receivable(row: &sqlx::sqlite::SqliteRow) -> AppResult<Receivable> {
        let status = ReceivableStatus::from_snake(row.get::<String, _>("status").as_str())
            .ok_or_else(|| infra("invalid receivable status"))?;

        Ok(Receivable {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,

            invoice_id: Uuid::parse_str(row.get::<String, _>("invoice_id").as_str())
                .map_err(infra)?,

            outstanding_amount: row
                .get::<String, _>("outstanding_amount")
                .parse()
                .map_err(infra)?,

            due_date: NaiveDate::parse_from_str(
                row.get::<String, _>("due_date").as_str(),
                "%Y-%m-%d",
            )
            .map_err(infra)?,

            status,
        })
    }
}
