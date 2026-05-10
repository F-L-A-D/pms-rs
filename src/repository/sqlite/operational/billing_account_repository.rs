use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::domain::billing_account::{
    BillingAccount,
    BillingAccountStatus,
};

pub struct SqliteBillingAccountRepository;

impl SqliteBillingAccountRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        account: &BillingAccount,
    ) -> Result<(), String> {

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
        .bind(format!("{:?}", account.status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> Result<Option<BillingAccount>, String> {

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
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {

            let status =
                match r.get::<String, _>("status").as_str() {

                    "Suspended" =>
                        BillingAccountStatus::Suspended,

                    _ =>
                        BillingAccountStatus::Active,
                };

            Ok(Some(
                BillingAccount {

                    id:
                        Uuid::parse_str(
                            r.get::<String, _>("id")
                                .as_str()
                        )
                        .unwrap(),

                    company_id:
                        r.get::<Option<String>, _>(
                            "company_id"
                        )
                        .map(|s| Uuid::parse_str(&s))
                        .transpose()
                        .map_err(|e| e.to_string())?,

                    name:
                        r.get("name"),

                    status,
                }
            ))

        } else {
            Ok(None)
        }
    }
}