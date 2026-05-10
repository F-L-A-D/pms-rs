use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::domain::company::{
    Company,
    CompanyStatus,
};

pub struct SqliteCompanyRepository;

impl SqliteCompanyRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        company: &Company,
    ) -> Result<(), String> {

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
        .bind(format!("{:?}", company.status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> Result<Option<Company>, String> {

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
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {

            let status =
                match r.get::<String, _>("status").as_str() {

                    "Inactive" =>
                        CompanyStatus::Inactive,

                    _ =>
                        CompanyStatus::Active,
                };

            Ok(Some(
                Company {
                    id:
                        Uuid::parse_str(
                            r.get::<String, _>("id")
                                .as_str()
                        )
                        .unwrap(),

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