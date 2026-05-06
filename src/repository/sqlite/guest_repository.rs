use sqlx::{
    Row,
    SqlitePool,
};

use crate::domain::guest::Guest;

pub struct SqliteGuestRepository;

impl SqliteGuestRepository {

    pub async fn save(
        db: &SqlitePool,
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
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        db: &SqlitePool,
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
            .fetch_optional(db)
            .await
            .map_err(|e| e.to_string())?;

        match row {

            Some(r) => {

                Ok(
                    Some(
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
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_all(
        db: &SqlitePool,
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
            .fetch_all(db)
            .await
            .map_err(|e| e.to_string())?;

        let mut guests = vec![];

        for r in rows {

            guests.push(
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
            );
        }

        Ok(guests)
    }

    pub async fn update(
        db: &SqlitePool,
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
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
    
    pub async fn find_by_name(
        db: &SqlitePool,
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
            .fetch_all(db)
            .await
            .map_err(|e| e.to_string())?;

        let mut guests = vec![];

        for r in rows {

            guests.push(
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
            );
        }

        Ok(guests)
    }
}