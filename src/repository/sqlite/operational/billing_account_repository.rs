use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::billing_account::{BillingAccount, BillingAccountStatus},
    error::app_error::{infra, AppResult},
};

pub struct SqliteBillingAccountRepository;

impl SqliteBillingAccountRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, account: &BillingAccount) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO billing_accounts (
                id,
                company_id,
                name,
                status,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(account.id.to_string())
        .bind(account.company_id.map(|id| id.to_string()))
        .bind(&account.name)
        .bind(account.status.to_snake())
        .bind(account.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<BillingAccount>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    company_id,
                    name,
                    status,
                    created_at
                FROM billing_accounts
                WHERE id = ?1
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_billing_account(&row)?)),

            None => Ok(None),
        }
    }

    fn row_to_billing_account(row: &sqlx::sqlite::SqliteRow) -> AppResult<BillingAccount> {
        let status = BillingAccountStatus::from_snake(row.get::<String, _>("status").as_str())
            .ok_or_else(|| infra("invalid billing account status"))?;

        let created_at =
            chrono::DateTime::parse_from_rfc3339(row.get::<String, _>("created_at").as_str())
                .map_err(infra)?
                .with_timezone(&chrono::Utc);

        Ok(BillingAccount {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,

            company_id: row
                .get::<Option<String>, _>("company_id")
                .map(|s| Uuid::parse_str(&s))
                .transpose()
                .map_err(infra)?,

            name: row.get("name"),

            status,

            created_at,
        })
    }
}
