use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::payment::{Payment, PaymentMethod},
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
                method,
                external_reference,
                paid_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(payment.id.to_string())
        .bind(payment.folio_id.to_string())
        .bind(payment.amount.to_string())
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
        id: Uuid,
    ) -> AppResult<Option<Payment>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    folio_id,
                    amount,
                    method,
                    external_reference,
                    paid_at
                FROM payments
                WHERE id = ?1
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_payment(&row)?)),
            None => Ok(None),
        }
    }

    fn row_to_payment(row: &sqlx::sqlite::SqliteRow) -> AppResult<Payment> {
        let method = PaymentMethod::from_snake(row.get::<String, _>("method").as_str())
            .ok_or_else(|| infra("invalid payment method"))?;

        let paid_at = DateTime::parse_from_rfc3339(row.get::<String, _>("paid_at").as_str())
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(Payment {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            folio_id: Uuid::parse_str(row.get::<String, _>("folio_id").as_str()).map_err(infra)?,
            amount: row.get::<String, _>("amount").parse().map_err(infra)?,
            method,
            external_reference: row.get("external_reference"),
            paid_at,
        })
    }
}
