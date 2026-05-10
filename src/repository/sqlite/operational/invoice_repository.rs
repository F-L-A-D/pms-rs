use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::domain::invoice::{
    Invoice,
    InvoiceStatus,
};

pub struct SqliteInvoiceRepository;

impl SqliteInvoiceRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        invoice: &Invoice,
    ) -> Result<(), String> {

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
        .bind(format!("{:?}", invoice.status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: Uuid,
    ) -> Result<Option<Invoice>, String> {

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
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {

            let status =
                match r.get::<String, _>("status").as_str() {

                    "Voided" =>
                        InvoiceStatus::Voided,

                    _ =>
                        InvoiceStatus::Issued,
                };

            Ok(Some(
                Invoice {

                    id:
                        Uuid::parse_str(
                            r.get::<String, _>("id")
                                .as_str()
                        )
                        .unwrap(),

                    folio_id:
                        Uuid::parse_str(
                            r.get::<String, _>("folio_id")
                                .as_str()
                        )
                        .unwrap(),

                    billing_account_id:
                        Uuid::parse_str(
                            r.get::<String, _>(
                                "billing_account_id"
                            )
                            .as_str()
                        )
                        .unwrap(),

                    issued_amount:
                        r.get("issued_amount"),

                    issued_at:
                        DateTime::parse_from_rfc3339(
                            r.get::<String, _>(
                                "issued_at"
                            )
                            .as_str()
                        )
                        .unwrap()
                        .with_timezone(&Utc),

                    status,
                }
            ))

        } else {

            Ok(None)
        }
    }
}