use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::guest::{Gender, Guest, GuestProfile, GuestSearchField},
    error::app_error::{infra, AppResult},
};

pub struct SqliteGuestRepository;

impl SqliteGuestRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, guest: &Guest) -> AppResult<()> {
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
            "#,
        )
        .bind(guest.id.to_string())
        .bind(&guest.profile.last_name)
        .bind(&guest.profile.first_name)
        .bind(&guest.profile.phone)
        .bind(&guest.profile.email)
        .bind(&guest.profile.nationality)
        .bind(guest.profile.birth_date.map(|v| v.to_string()))
        .bind(guest.profile.gender.as_ref().map(Gender::to_snake))
        .bind(&guest.profile.membership_code)
        .bind(&guest.profile.marketing_opt_in)
        .bind(guest.created_at.to_rfc3339())
        .bind(guest.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn update(tx: &mut Transaction<'_, Sqlite>, guest: &Guest) -> AppResult<()> {
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
            "#,
        )
        .bind(&guest.profile.last_name)
        .bind(&guest.profile.first_name)
        .bind(&guest.profile.phone)
        .bind(&guest.profile.email)
        .bind(&guest.profile.nationality)
        .bind(guest.profile.birth_date.map(|v| v.to_string()))
        .bind(guest.profile.gender.as_ref().map(Gender::to_snake))
        .bind(&guest.profile.membership_code)
        .bind(guest.profile.marketing_opt_in)
        .bind(guest.updated_at.to_rfc3339())
        .bind(guest.id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Guest>> {
        let row = sqlx::query(
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
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(r) => Ok(Some(Self::row_to_guest(&r)?)),

            None => Ok(None),
        }
    }

    pub async fn list_guests(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Guest>> {
        let rows = sqlx::query(
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
                    first_name,
                    updated_at DESC
                LIMIT 50
                "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(rows
            .iter()
            .map(Self::row_to_guest)
            .collect::<AppResult<Vec<_>>>()?)
    }

    pub async fn search(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
        field: Option<GuestSearchField>,
    ) -> AppResult<Vec<Guest>> {
        let tokens = keyword.split_whitespace().collect::<Vec<_>>();

        let rows = if field.is_none() && tokens.len() >= 2 {
            let last_name = format!("%{}%", tokens[0],);

            let first_name = format!("%{}%", tokens[1],);

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
                    "#,
            )
            .bind(&last_name)
            .bind(&first_name)
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?
        } else {
            let pattern = format!("%{}%", keyword,);

            let sql = match field {
                Some(GuestSearchField::FirstName) => {
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
                            WHERE first_name LIKE ?1
                            ORDER BY
                                last_name,
                                first_name
                            LIMIT 50
                            "#
                }

                Some(GuestSearchField::LastName) => {
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
                            WHERE last_name LIKE ?1
                            ORDER BY
                                last_name,
                                first_name
                            LIMIT 50
                            "#
                }

                Some(GuestSearchField::Email) => {
                    r#"
                            ...
                            WHERE email LIKE ?1
                            "#
                }

                Some(GuestSearchField::Phone) => {
                    r#"
                            ...
                            WHERE phone LIKE ?1
                            "#
                }

                Some(GuestSearchField::MembershipCode) => {
                    r#"
                            ...
                            WHERE membership_code LIKE ?1
                            "#
                }

                _ => {
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
                .map_err(infra)?
        };

        Ok(rows
            .iter()
            .map(Self::row_to_guest)
            .collect::<AppResult<Vec<_>>>()?)
    }

    fn row_to_guest(row: &sqlx::sqlite::SqliteRow) -> AppResult<Guest> {
        let id = Uuid::parse_str(row.try_get::<String, _>("id").map_err(infra)?.as_str())
            .map_err(infra)?;

        let gender = row
            .try_get::<Option<String>, _>("gender")
            .map_err(infra)?
            .map(|v| Gender::from_snake(v.as_str()).ok_or_else(|| infra("invalid gender")))
            .transpose()?;

        let birth_date = row
            .try_get::<Option<String>, _>("birth_date")
            .map_err(infra)?
            .map(|v| v.parse())
            .transpose()
            .map_err(infra)?;

        let created_at = chrono::DateTime::parse_from_rfc3339(
            row.try_get::<String, _>("created_at")
                .map_err(infra)?
                .as_str(),
        )
        .map_err(infra)?
        .with_timezone(&chrono::Utc);

        let updated_at = chrono::DateTime::parse_from_rfc3339(
            row.try_get::<String, _>("updated_at")
                .map_err(infra)?
                .as_str(),
        )
        .map_err(infra)?
        .with_timezone(&chrono::Utc);

        Ok(Guest {
            id,

            profile: GuestProfile {
                last_name: row.try_get("last_name").map_err(infra)?,

                first_name: row.try_get("first_name").map_err(infra)?,

                phone: row.try_get("phone").map_err(infra)?,

                email: row.try_get("email").map_err(infra)?,

                nationality: row.try_get("nationality").map_err(infra)?,

                birth_date,

                gender,

                membership_code: row.try_get("membership_code").map_err(infra)?,

                marketing_opt_in: row.try_get("marketing_opt_in").map_err(infra)?,
            },

            created_at,

            updated_at,
        })
    }
}
