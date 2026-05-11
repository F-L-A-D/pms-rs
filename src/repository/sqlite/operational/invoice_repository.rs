use chrono::{
    DateTime,
    Utc,
};

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::invoice::{
        Invoice,
        InvoiceStatus,
    },

    error::app_error::{
        AppResult,
        infra,
    },
};

pub struct SqliteInvoiceRepository;

impl SqliteInvoiceRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        invoice: &Invoice,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO invoices (
                id,
                folio_id,
                billing_account_id,
                issued_amount,
                issued_at,
                status
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#
        )
        .bind(invoice.id.to_string())
        .bind(invoice.folio_id.to_string())
        .bind(invoice.billing_account_id.to_string())
        .bind(invoice.issued_amount)
        .bind(invoice.issued_at.to_rfc3339())
        .bind(
            match invoice.status {

                InvoiceStatus::Issued =>
                    "Issued",

                InvoiceStatus::Voided =>
                    "Voided",
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
    ) -> AppResult<Option<Invoice>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    folio_id,
                    billing_account_id,
                    issued_amount,
                    issued_at,
                    status
                FROM invoices
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
                        Self::row_to_invoice(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: Uuid,
    ) -> AppResult<Option<Invoice>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    folio_id,
                    billing_account_id,
                    issued_amount,
                    issued_at,
                    status
                FROM invoices
                WHERE folio_id = ?1
                "#
            )
            .bind(folio_id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        match row {

            Some(row) => {
                Ok(
                    Some(
                        Self::row_to_invoice(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn list_by_billing_account_id(
        tx: &mut Transaction<'_, Sqlite>,
        billing_account_id: Uuid,
    ) -> AppResult<Vec<Invoice>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    folio_id,
                    billing_account_id,
                    issued_amount,
                    issued_at,
                    status
                FROM invoices
                WHERE billing_account_id = ?1
                ORDER BY issued_at DESC
                "#
            )
            .bind(
                billing_account_id.to_string()
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(
            rows.iter()
                .map(Self::row_to_invoice)
                .collect::<AppResult<Vec<_>>>()?
        )
    }

    fn row_to_invoice(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Invoice> {

        let status =
            match row
                .get::<String, _>("status")
                .as_str()
            {

                "Issued" =>
                    InvoiceStatus::Issued,

                "Voided" =>
                    InvoiceStatus::Voided,

                _ => {
                    return Err(
                        infra(
                            "invalid invoice status"
                        )
                    )
                }
            };

        let issued_at =
            DateTime::parse_from_rfc3339(
                row.get::<String, _>("issued_at")
                    .as_str()
            )
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(
            Invoice {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                folio_id:
                    Uuid::parse_str(
                        row.get::<String, _>("folio_id")
                            .as_str()
                    )
                    .map_err(infra)?,

                billing_account_id:
                    Uuid::parse_str(
                        row.get::<String, _>(
                            "billing_account_id"
                        )
                        .as_str()
                    )
                    .map_err(infra)?,

                issued_amount:
                    row.get("issued_amount"),

                issued_at,

                status,
            }
        )
    }
}