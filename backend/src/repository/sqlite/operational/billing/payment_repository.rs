use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::payment::{Payment, PaymentMethod, PaymentStatus},
    error::app_error::{infra, AppResult},
};

pub struct SqlitePaymentRepository;

impl SqlitePaymentRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, payment: &Payment) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO payments (
                id,
                folio_id,
                amount,
                unapplied_amount,
                refunded_amount,
                status,
                method,
                external_reference,
                paid_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                folio_id = excluded.folio_id,
                amount = excluded.amount,
                unapplied_amount = excluded.unapplied_amount,
                refunded_amount = excluded.refunded_amount,
                status = excluded.status,
                method = excluded.method,
                external_reference = excluded.external_reference,
                paid_at = excluded.paid_at
            "#,
        )
        .bind(payment.id.to_string())
        .bind(payment.folio_id.to_string())
        .bind(payment.amount.to_string())
        .bind(payment.unapplied_amount.to_string())
        .bind(payment.refunded_amount.to_string())
        .bind(payment.status.to_snake())
        .bind(payment.method.to_snake())
        .bind(&payment.external_reference)
        .bind(payment.paid_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        payment_id: Uuid,
    ) -> AppResult<Option<Payment>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    folio_id,
                    amount,
                    unapplied_amount,
                    refunded_amount,
                    status,
                    method,
                    external_reference,
                    paid_at
                FROM payments
                WHERE id = ?1
                "#,
        )
        .bind(payment_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(row_to_payment).transpose()
    }

    pub async fn list_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: Uuid,
    ) -> AppResult<Vec<Payment>> {
        let rows = sqlx::query(
            r#"
                SELECT
                    id,
                    folio_id,
                    amount,
                    unapplied_amount,
                    refunded_amount,
                    status,
                    method,
                    external_reference,
                    paid_at
                FROM payments
                WHERE folio_id = ?1
                ORDER BY paid_at ASC
                "#,
        )
        .bind(folio_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.into_iter().map(row_to_payment).collect()
    }
}

fn row_to_payment(row: sqlx::sqlite::SqliteRow) -> AppResult<Payment> {
    let id: String = row.try_get("id").map_err(infra)?;

    let folio_id: String = row.try_get("folio_id").map_err(infra)?;

    let amount: String = row.try_get("amount").map_err(infra)?;

    let unapplied_amount: String = row.try_get("unapplied_amount").map_err(infra)?;

    let refunded_amount: String = row.try_get("refunded_amount").map_err(infra)?;

    let status: String = row.try_get("status").map_err(infra)?;

    let method: String = row.try_get("method").map_err(infra)?;

    let external_reference: Option<String> = row.try_get("external_reference").map_err(infra)?;

    let paid_at: String = row.try_get("paid_at").map_err(infra)?;

    let status =
        PaymentStatus::from_snake(&status).ok_or_else(|| infra("invalid payment status"))?;

    let method =
        PaymentMethod::from_snake(&method).ok_or_else(|| infra("invalid payment method"))?;

    Ok(Payment {
        id: Uuid::parse_str(&id).map_err(infra)?,
        folio_id: Uuid::parse_str(&folio_id).map_err(infra)?,
        amount: amount.parse::<Decimal>().map_err(infra)?,
        unapplied_amount: unapplied_amount.parse::<Decimal>().map_err(infra)?,
        refunded_amount: refunded_amount.parse::<Decimal>().map_err(infra)?,
        status,
        method,
        external_reference,
        paid_at: DateTime::parse_from_rfc3339(&paid_at)
            .map_err(infra)?
            .with_timezone(&Utc),
    })
}
