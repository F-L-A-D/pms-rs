use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    domain::company::{
        Company,
        CompanyStatus,
    },

    error::app_error::{
        AppResult,
        infra,
    },
};

pub struct SqliteCompanyRepository;

impl SqliteCompanyRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        company: &Company,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO companies (
                id,
                name,
                status
            )
            VALUES (?1, ?2, ?3)
            "#
        )
        .bind(company.id.to_string())
        .bind(&company.name)
        .bind(
            match company.status {

                CompanyStatus::Active =>
                    "Active",

                CompanyStatus::Inactive =>
                    "Inactive",
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
    ) -> AppResult<Option<Company>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    name,
                    status
                FROM companies
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
                        Self::row_to_company(&row)?
                    )
                )
            }

            None => Ok(None),
        }
    }

    fn row_to_company(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Company> {

        let status =
            match row
                .get::<String, _>("status")
                .as_str()
            {

                "Active" =>
                    CompanyStatus::Active,

                "Inactive" =>
                    CompanyStatus::Inactive,

                _ => {
                    return Err(
                        infra(
                            "invalid company status"
                        )
                    )
                }
            };

        Ok(
            Company {

                id:
                    Uuid::parse_str(
                        row.get::<String, _>("id")
                            .as_str()
                    )
                    .map_err(infra)?,

                name:
                    row.get("name"),

                status,
            }
        )
    }
}