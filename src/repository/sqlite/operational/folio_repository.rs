use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::domain::folio::{Folio, FolioStatus};

pub struct SqliteFolioRepository;

impl SqliteFolioRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, folio: &Folio) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO folios (
                id,
                reservation_id,
                status
            )
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(&folio.id.to_string())
        .bind(folio.reservation_id.to_string())
        .bind(format!("{:?}", folio.status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> Result<Option<Folio>, String> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    reservation_id,
                    status
                FROM folios
                WHERE id = ?1
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let status = match r.get::<String, _>("status").as_str() {
                "Open" => FolioStatus::Open,
                "Closed" => FolioStatus::Closed,
                _ => return Err("invalid folio status".into()),
            };

            Ok(Some(Folio {
                id: Uuid::parse_str(
                    r.get::<String, _>("id")
                        .as_str()
                )
                .unwrap(),

                reservation_id: Uuid::parse_str(
                    r.get::<String, _>("reservation_id").as_str(),
                )
                .map_err(|e| e.to_string())?,
                status,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> Result<Vec<Folio>, String> {
        let rows = sqlx::query(
            r#"
                SELECT
                    id,
                    reservation_id,
                    status
                FROM folios
                WHERE reservation_id = ?1
                "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut folios = vec![];

        for r in rows {
            let status = match r.get::<String, _>("status").as_str() {
                "Closed" => crate::domain::folio::FolioStatus::Closed,

                _ => crate::domain::folio::FolioStatus::Open,
            };

            folios.push(Folio {
                id: Uuid::parse_str(
                    r.get::<String, _>("id")
                        .as_str()
                )
                .unwrap(),

                reservation_id: Uuid::parse_str(
                    r.get::<String, _>("reservation_id").as_str(),
                )
                .map_err(|e| e.to_string())?,

                status,
            });
        }

        Ok(folios)
    }
}
