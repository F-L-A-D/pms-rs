use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_note::{ReservationNote, ReservationNoteKind},
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationNoteRepository;

impl SqliteReservationNoteRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, note: &ReservationNote) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_notes (
                id,
                reservation_id,
                kind,
                department_code,
                body,
                actor_id,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(note.id.to_string())
        .bind(note.reservation_id.to_string())
        .bind(note.kind.to_snake())
        .bind(&note.department_code)
        .bind(&note.body)
        .bind(&note.actor_id)
        .bind(note.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationNote>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                kind,
                department_code,
                body,
                actor_id,
                created_at
            FROM reservation_notes
            WHERE reservation_id = ?1
            ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_note).collect()
    }

    fn row_to_note(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationNote> {
        let kind = ReservationNoteKind::from_snake(row.get::<String, _>("kind").as_str())
            .ok_or_else(|| infra("invalid reservation note kind"))?;

        Ok(ReservationNote {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            kind,
            department_code: row.get("department_code"),
            body: row.get("body"),
            actor_id: row.get("actor_id"),
            created_at: parse_datetime(row.get::<String, _>("created_at").as_str())?,
        })
    }
}

fn parse_datetime(value: &str) -> AppResult<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .map_err(infra)?
        .with_timezone(&Utc))
}
