use sqlx::{Row, Sqlite, Transaction};

use crate::domain::guest::Guest;

pub struct SqliteGuestRepository;

impl SqliteGuestRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        guest: &Guest,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT INTO guests (
                id,
                last_name,
                first_name,
                phone,
                email,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#
        )
        .bind(&guest.id)
        .bind(&guest.last_name)
        .bind(&guest.first_name)
        .bind(&guest.phone)
        .bind(&guest.email)
        .bind(guest.created_at.to_rfc3339())
        .bind(guest.updated_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: &str,
    ) -> Result<Option<Guest>, String> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
                    created_at,
                    updated_at
                FROM guests
                WHERE id = ?1
                "#
            )
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

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
    ) -> Result<Vec<Guest>, String> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
                    created_at,
                    updated_at
                FROM guests
                ORDER BY last_name, first_name
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

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
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            UPDATE guests
            SET
                last_name = ?1,
                first_name = ?2,
                phone = ?3,
                email = ?4,
                updated_at = ?5
            WHERE id = ?6
            "#
        )
        .bind(&guest.last_name)
        .bind(&guest.first_name)
        .bind(&guest.phone)
        .bind(&guest.email)
        .bind(guest.updated_at.to_rfc3339())
        .bind(&guest.id)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_name(
        tx: &mut Transaction<'_, Sqlite>,
        keyword: &str,
    ) -> Result<Vec<Guest>, String> {

        let pattern = format!("%{}%", keyword);

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    last_name,
                    first_name,
                    phone,
                    email,
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
            .map_err(|e| e.to_string())?;

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
            id: r.get("id"),
            last_name: r.get("last_name"),
            first_name: r.get("first_name"),
            phone: r.get("phone"),
            email: r.get("email"),
            created_at: r
                .get::<String, _>("created_at")
                .parse()
                .unwrap(),
            updated_at: r
                .get::<String, _>("updated_at")
                .parse()
                .unwrap(),
        }
    }
}