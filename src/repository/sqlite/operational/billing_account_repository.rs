use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    domain::billing_account::{
        BillingAccount,
        BillingAccountStatus,
    },
};

pub struct SqliteBillingAccountRepository;

impl SqliteBillingAccountRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        account: &BillingAccount,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO billing_accounts (
                id,
                company_id,
                name,
                status
            )
            VALUES (?1, ?2, ?3, ?4)
            "#
        )
        .bind(account.id.to_string())
        .bind(
            account.company_id
                .map(|id| id.to_string())
        )
        .bind(&account.name)
        .bind(
            match account.status {

                BillingAccountStatus::Active =>
                    "Active",

                BillingAccountStatus::Suspended =>
                    "Suspended",
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
    ) -> AppResult<Option<BillingAccount>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    company_id,
                    name,
                    status
                FROM billing_accounts
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
                        Self::row_to_billing_account(
                            &row
                        )?
                    )
                )
            }

            None => Ok(None),
        }
    }

    fn row_to_billing_account(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<BillingAccount> {

        let status =
            match row
                .get::<String, _>("status")
                .as_str()
            {

                "Active" =>
                    BillingAccountStatus::Active,

                "Suspended" =>
                    BillingAccountStatus::Suspended,

                _ => {
                    return Err(
                        infra(
                            "invalid billing account status"
                        )
                    )
                }
            };

        Ok(
            BillingAccount {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                company_id:
                    row.get::<Option<String>, _>(
                        "company_id"
                    )
                    .map(|s| {
                        Uuid::parse_str(&s)
                    })
                    .transpose()
                    .map_err(infra)?,

                name:
                    row.get("name"),

                status,
            }
        )
    }
}