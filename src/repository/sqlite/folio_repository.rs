use sqlx::{Row, SqlitePool};
use crate::domain::folio::{
    Folio,
    FolioStatus,
};

pub struct SqliteFolioRepository;

impl SqliteFolioRepository {
    pub async  fn save(
        db: &SqlitePool,
        folio: &Folio,
    ) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO folios (
                id,
                reservation_id,
                status
            )
            VALUES (?1, ?2, ?3)
            "#
        )
        .bind(&folio.id)
        .bind(&folio.reservation_id)
        .bind(format!("{:?}", folio.status))
        .execute(db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        db: &SqlitePool,
        id: &str,
    ) -> Result<Option<Folio>, String> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    reservation_id,
                    status
                FROM folios
                WHERE id = ?1
            "#
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let status = match r.get::<String, _>("status").as_str() {
                "Open" => FolioStatus::Open,
                "Closed" => FolioStatus::Closed,
                _ => return Err("invalid folio status".into()),
            };

            Ok(Some(Folio{
                id: r.get("id"),
                reservation_id: r.get("reservation_id"),
                status,
            }))
        } else {
            Ok(None)
        }
    }
}