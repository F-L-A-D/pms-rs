use sqlx::{Row, Transaction, Sqlite};
use crate::domain::folio::{
    Folio,
    FolioStatus,
};

pub struct SqliteFolioRepository;

impl SqliteFolioRepository {
    pub async  fn save(
        tx: &mut Transaction<'_, Sqlite>,
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
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
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
        .fetch_optional(&mut **tx)
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

    pub async fn find_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: &str,
    ) -> Result<Vec<Folio>, String> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    id,
                    reservation_id,
                    status
                FROM folios
                WHERE reservation_id = ?1
                "#
            )
            .bind(reservation_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        let mut folios = vec![];

        for r in rows {

            let status =
                match r.get::<String, _>("status").as_str() {

                    "Closed" =>
                        crate::domain::folio::FolioStatus::Closed,

                    _ =>
                        crate::domain::folio::FolioStatus::Open,
                };

            folios.push(
                Folio {
                    id: r.get("id"),

                    reservation_id:
                        r.get("reservation_id"),

                    status,
                }
            );
        }

        Ok(folios)
    }
}