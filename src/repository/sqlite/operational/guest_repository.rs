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

    pub async fn list(
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
                ORDER BY
                    last_name,
                    first_name
                LIMIT 50
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

    pub async fn search(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
        field: Option<&str>,
    ) -> AppResult<Vec<Guest>> {

        let tokens =
            keyword
                .split_whitespace()
                .collect::<Vec<_>>();

        let rows =

            if field.is_none()
                && tokens.len() >= 2 {

                let last_name =
                    format!(
                        "%{}%",
                        tokens[0],
                    );

                let first_name =
                    format!(
                        "%{}%",
                        tokens[1],
                    );

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
                        AND first_name LIKE ?2
                    ORDER BY
                        last_name,
                        first_name
                    LIMIT 50
                    "#
                )
                .bind(&last_name)
                .bind(&first_name)
                .fetch_all(&mut **tx)
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?

            } else {

                let pattern =
                    format!(
                        "%{}%",
                        keyword,
                    );

                let sql =
                    match field {

                        Some("first_name") => {
                            r#"
                            SELECT * FROM guests
                            WHERE first_name LIKE ?1
                            ORDER BY last_name, first_name
                            LIMIT 50
                            "#
                        }

                        Some("last_name") => {
                            r#"
                            SELECT * FROM guests
                            WHERE last_name LIKE ?1
                            ORDER BY last_name, first_name
                            LIMIT 50
                            "#
                        }

                        Some("email") => {
                            r#"
                            SELECT * FROM guests
                            WHERE email LIKE ?1
                            ORDER BY last_name, first_name
                            LIMIT 50
                            "#
                        }

                        Some("phone") => {
                            r#"
                            SELECT * FROM guests
                            WHERE phone LIKE ?1
                            ORDER BY last_name, first_name
                            LIMIT 50
                            "#
                        }

                        Some("membership_code") => {
                            r#"
                            SELECT * FROM guests
                            WHERE membership_code LIKE ?1
                            ORDER BY last_name, first_name
                            LIMIT 50
                            "#
                        }

                        _ => {
                            r#"
                            SELECT * FROM guests
                            WHERE
                                first_name LIKE ?1
                                OR last_name LIKE ?1
                                OR email LIKE ?1
                                OR phone LIKE ?1
                                OR membership_code LIKE ?1
                            ORDER BY
                                last_name,
                                first_name
                            LIMIT 50
                            "#
                        }
                    };

                sqlx::query(sql)
                    .bind(&pattern)
                    .fetch_all(&mut **tx)
                    .await
                    .map_err(|e| {
                        AppError::Infrastructure(
                            e.to_string()
                        )
                    })?
            };

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
                r.get::<Option<String>, _>(
                    "gender"
                )
                .map(|v| {
                    match v.as_str() {

                        "Male" =>
                            Gender::Male,

                        "Female" =>
                            Gender::Female,

                        _ =>
                            Gender::Other,
                    }
                }),

            membership_code:
                r.get("membership_code"),

            marketing_opt_in:
                r.get("marketing_opt_in"),

            created_at:
                chrono::DateTime::parse_from_rfc3339(
                    r.get::<String, _>(
                        "created_at"
                    )
                    .as_str()
                )
                .unwrap()
                .with_timezone(&chrono::Utc),

            updated_at:
                chrono::DateTime::parse_from_rfc3339(
                    r.get::<String, _>(
                        "updated_at"
                    )
                    .as_str()
                )
                .unwrap()
                .with_timezone(&chrono::Utc),
        }
    }
}