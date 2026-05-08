use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use chrono::NaiveDate;

use uuid::Uuid;

use crate::domain::guest::{
    Gender,
    Guest,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

pub struct SqliteGuestRepository;

impl SqliteGuestRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        guest: &Guest,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            INSERT INTO guests (
                id,
                last_name,
                first_name,
                phone,
                email,
                nationality,
                birth_date,
                gender,
                membership_code,
                marketing_opt_in,
                created_at,
                updated_at
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8,
                ?9,
                ?10,
                ?11,
                ?12
            )
            "#
        )
        .bind(&guest.id.to_string())
        .bind(&guest.last_name)
        .bind(&guest.first_name)
        .bind(&guest.phone)
        .bind(&guest.email)
        .bind(&guest.nationality)
        .bind(guest.birth_date)
        .bind(
            guest.gender.as_ref().map(|v| {
                format!("{:?}", v)
            })
        )
        .bind(&guest.membership_code)
        .bind(guest.marketing_opt_in)
        .bind(guest.created_at.to_rfc3339())
        .bind(guest.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Guest>> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
                    nationality,
                    birth_date,
                    gender,
                    membership_code,
                    marketing_opt_in,
                    created_at,
                    updated_at
                FROM guests
                WHERE id = ?1
                "#
            )
            .bind(id.to_string())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        match row {

            Some(r) => {
                Ok(
                    Some(
                        Self::row_to_guest(r)
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Vec<Guest>> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
                    nationality,
                    birth_date,
                    gender,
                    membership_code,
                    marketing_opt_in,
                    created_at,
                    updated_at
                FROM guests
                ORDER BY last_name, first_name
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        Ok(
            rows
                .into_iter()
                .map(Self::row_to_guest)
                .collect()
        )
    }

    pub async fn update(
        tx: &mut Transaction<'_, Sqlite>,
        guest: &Guest,
    ) -> AppResult<()> {

        sqlx::query(
            r#"
            UPDATE guests
            SET
                last_name = ?1,
                first_name = ?2,
                phone = ?3,
                email = ?4,
                nationality = ?5,
                birth_date = ?6,
                gender = ?7,
                membership_code = ?8,
                marketing_opt_in = ?9,
                updated_at = ?10
            WHERE id = ?11
            "#
        )
        .bind(&guest.last_name)
        .bind(&guest.first_name)
        .bind(&guest.phone)
        .bind(&guest.email)
        .bind(&guest.nationality)
        .bind(guest.birth_date)
        .bind(
            guest.gender.as_ref().map(|v| {
                format!("{:?}", v)
            })
        )
        .bind(&guest.membership_code)
        .bind(guest.marketing_opt_in)
        .bind(guest.updated_at.to_rfc3339())
        .bind(&guest.id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

        Ok(())
    }

    pub async fn find_by_name(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
    ) -> AppResult<Vec<Guest>> {

        let pattern =
            format!("%{}%", keyword);

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
                    nationality,
                    birth_date,
                    gender,
                    membership_code,
                    marketing_opt_in,
                    created_at,
                    updated_at
                FROM guests
                WHERE
                    last_name LIKE ?1
                    OR first_name LIKE ?1
                ORDER BY last_name, first_name
                "#
            )
            .bind(pattern)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                AppError::Infrastructure(
                    e.to_string()
                )
            })?;

        Ok(
            rows
                .into_iter()
                .map(Self::row_to_guest)
                .collect()
        )
    }

    fn row_to_guest(
        r: sqlx::sqlite::SqliteRow,
    ) -> Guest {

        Guest {
            id:
                Uuid::parse_str(
                    r.get::<String, _>("id")
                        .as_str()
                )
                .unwrap(),

            last_name:
                r.get("last_name"),

            first_name:
                r.get("first_name"),

            phone:
                r.get("phone"),

            email:
                r.get("email"),

            nationality:
                r.get("nationality"),

            birth_date:
                r.get::<Option<NaiveDate>, _>(
                    "birth_date"
                ),

            gender:
                match r.get::<Option<String>, _>(
                    "gender"
                ) {

                    Some(v) => {
                        match v.as_str() {

                            "Male" =>
                                Some(Gender::Male),

                            "Female" =>
                                Some(Gender::Female),

                            "Other" =>
                                Some(Gender::Other),

                            "Unspecified" => {
                                Some(
                                    Gender::Unspecified
                                )
                            }

                            _ => None,
                        }
                    }

                    None => None,
                },

            membership_code:
                r.get("membership_code"),

            marketing_opt_in:
                r.get("marketing_opt_in"),

            created_at:
                r.get::<String, _>("created_at")
                    .parse()
                    .unwrap(),

            updated_at:
                r.get::<String, _>("updated_at")
                    .parse()
                    .unwrap(),
        }
    }
}