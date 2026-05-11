use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::receivable::{
        Receivable,
        ReceivableStatus,
    },

    error::app_error::{
        AppResult,
        infra,
    },
};

pub struct SqliteReceivableRepository;

impl SqliteReceivableRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        receivable: &Receivable,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO receivables (
                id,
                invoice_id,
                outstanding_amount,
                status
            )
            VALUES (?1, ?2, ?3, ?4)
            "#
        )
        .bind(receivable.id.to_string())
        .bind(receivable.invoice_id.to_string())
        .bind(receivable.outstanding_amount)
        .bind(
            match receivable.status {

                ReceivableStatus::Open =>
                    "OPEN",

                ReceivableStatus::Settled =>
                    "SETTLED",
                    
                ReceivableStatus::Disputed =>
                    "DISPUTED",
                    
                ReceivableStatus::WrittenOff =>
                    "WRITTENOFF",
            }
        )
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Receivable>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    invoice_id,
                    outstanding_amount,
                    status
                FROM receivables
                WHERE id = ?1
                "#
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_receivable(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_invoice_id(
        tx: &mut Transaction<'_, Sqlite>,
        invoice_id: Uuid,
    ) -> AppResult<Option<Receivable>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    invoice_id,
                    outstanding_amount,
                    status
                FROM receivables
                WHERE invoice_id = ?1
                "#
            )
            .bind(invoice_id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_receivable(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    fn row_to_receivable(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Receivable> {

        let status =
            match row
                .get::<String, _>("status")
                .as_str()
            {

                "OPEN" =>
                    ReceivableStatus::Open,

                "SETTLED" =>
                    ReceivableStatus::Settled,

                _ => {
                    return Err(
                        infra(
                            "invalid receivable status"
                        )
                    )
                }
            };

        Ok(
            Receivable {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                invoice_id:
                    Uuid::parse_str(
                        row.get::<String, _>("invoice_id")
                            .as_str()
                    )
                    .map_err(infra)?,

                outstanding_amount:
                    row.get(
                        "outstanding_amount"
                    ),

                status,
            }
        )
    }
}