use chrono::{
    DateTime,
    Utc,
};

use rust_decimal::Decimal;

use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::entity::{
        deposit::{
            Deposit,
            DepositStatus,
        },
        payment::PaymentMethod,
    },
    error::app_error::{
        infra,
        AppResult,
    },
};

pub struct SqliteDepositRepository;

impl SqliteDepositRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        deposit: &Deposit,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO deposits (
                id,
                folio_id,
                amount,
                unapplied_amount,
                refunded_amount,
                status,
                method,
                external_reference,
                received_at
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
                received_at = excluded.received_at
            "#,
        )
        .bind(deposit.id.to_string())
        .bind(deposit.folio_id.to_string())
        .bind(deposit.amount.to_string())
        .bind(deposit.unapplied_amount.to_string())
        .bind(deposit.refunded_amount.to_string())
        .bind(deposit.status.to_snake())
        .bind(deposit.method.to_snake())
        .bind(&deposit.external_reference)
        .bind(deposit.received_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        deposit_id: Uuid,
    ) -> AppResult<Option<Deposit>> {
        let row =
            sqlx::query(
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
                    received_at
                FROM deposits
                WHERE id = ?1
                "#,
            )
            .bind(deposit_id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(infra)?;

        row.map(row_to_deposit)
            .transpose()
    }

    pub async fn list_by_folio_id(
        tx: &mut Transaction<'_, Sqlite>,
        folio_id: Uuid,
    ) -> AppResult<Vec<Deposit>> {
        let rows =
            sqlx::query(
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
                    received_at
                FROM deposits
                WHERE folio_id = ?1
                ORDER BY received_at ASC
                "#,
            )
            .bind(folio_id.to_string())
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        rows.into_iter()
            .map(row_to_deposit)
            .collect()
    }
}

fn row_to_deposit(
    row: sqlx::sqlite::SqliteRow,
) -> AppResult<Deposit> {
    let id: String =
        row.try_get("id").map_err(infra)?;

    let folio_id: String =
        row.try_get("folio_id").map_err(infra)?;

    let amount: String =
        row.try_get("amount").map_err(infra)?;

    let unapplied_amount: String =
        row.try_get("unapplied_amount").map_err(infra)?;

    let refunded_amount: String =
        row.try_get("refunded_amount").map_err(infra)?;

    let status: String =
        row.try_get("status").map_err(infra)?;

    let method: String =
        row.try_get("method").map_err(infra)?;

    let external_reference: Option<String> =
        row.try_get("external_reference").map_err(infra)?;

    let received_at: String =
        row.try_get("received_at").map_err(infra)?;

    let status =
        DepositStatus::from_snake(&status)
            .ok_or_else(|| infra("invalid deposit status"))?;

    let method =
        PaymentMethod::from_snake(&method)
            .ok_or_else(|| infra("invalid payment method"))?;

    Ok(Deposit {
        id: Uuid::parse_str(&id).map_err(infra)?,
        folio_id: Uuid::parse_str(&folio_id).map_err(infra)?,
        amount: amount.parse::<Decimal>().map_err(infra)?,
        unapplied_amount:
            unapplied_amount
                .parse::<Decimal>()
                .map_err(infra)?,
        refunded_amount:
            refunded_amount
                .parse::<Decimal>()
                .map_err(infra)?,
        status,
        method,
        external_reference,
        received_at: DateTime::parse_from_rfc3339(&received_at)
            .map_err(infra)?
            .with_timezone(&Utc),
    })
}