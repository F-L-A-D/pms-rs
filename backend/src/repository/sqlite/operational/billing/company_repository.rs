use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::company::Company,
    error::app_error::{infra, AppResult},
};

pub struct SqliteCompanyRepository;

impl SqliteCompanyRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, company: &Company) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO companies (
                id,
                legal_name,
                tax_id,
                is_active,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(company.id.to_string())
        .bind(&company.legal_name)
        .bind(&company.tax_id)
        .bind(company.is_active)
        .bind(company.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Company>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    legal_name,
                    tax_id,
                    is_active,
                    created_at
                FROM companies
                WHERE id = ?1
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_company(&row)?)),

            None => Ok(None),
        }
    }

    fn row_to_company(row: &sqlx::sqlite::SqliteRow) -> AppResult<Company> {
        let created_at =
            chrono::DateTime::parse_from_rfc3339(row.get::<String, _>("created_at").as_str())
                .map_err(infra)?
                .with_timezone(&chrono::Utc);

        Ok(Company {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,

            legal_name: row.get("legal_name"),

            tax_id: row.get("tax_id"),

            is_active: row.get("is_active"),

            created_at,
        })
    }
}
