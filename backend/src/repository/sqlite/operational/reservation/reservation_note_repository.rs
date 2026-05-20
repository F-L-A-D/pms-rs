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
                body,
                actor_id,
                created_at,
                deleted_at,
                deleted_by
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
        )
        .bind(note.id.to_string())
        .bind(note.reservation_id.to_string())
        .bind(note.kind.to_snake())
        .bind(&note.body)
        .bind(&note.actor_id)
        .bind(note.created_at.to_rfc3339())
        .bind(note.deleted_at.map(|value| value.to_rfc3339()))
        .bind(&note.deleted_by)
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
                body,
                actor_id,
                created_at,
                deleted_at,
                deleted_by
            FROM reservation_notes
            WHERE reservation_id = ?1
              AND deleted_at IS NULL
            ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_note).collect()
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        note_id: Uuid,
    ) -> AppResult<Option<ReservationNote>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                kind,
                body,
                actor_id,
                created_at,
                deleted_at,
                deleted_by
            FROM reservation_notes
            WHERE id = ?1
            "#,
        )
        .bind(note_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(|row| Self::row_to_note(&row)).transpose()
    }

    pub async fn mark_deleted(
        tx: &mut Transaction<'_, Sqlite>,
        note_id: Uuid,
        deleted_at: DateTime<Utc>,
        deleted_by: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE reservation_notes
            SET
                deleted_at = ?1,
                deleted_by = ?2
            WHERE id = ?3
              AND deleted_at IS NULL
            "#,
        )
        .bind(deleted_at.to_rfc3339())
        .bind(deleted_by)
        .bind(note_id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    fn row_to_note(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationNote> {
        let kind = ReservationNoteKind::from_snake(row.get::<String, _>("kind").as_str())
            .ok_or_else(|| infra("invalid reservation note kind"))?;

        Ok(ReservationNote {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            kind,
            body: row.get("body"),
            actor_id: row.get("actor_id"),
            created_at: parse_datetime(row.get::<String, _>("created_at").as_str())?,
            deleted_at: row
                .get::<Option<String>, _>("deleted_at")
                .map(|value| parse_datetime(value.as_str()))
                .transpose()?,
            deleted_by: row.get("deleted_by"),
        })
    }
}

fn parse_datetime(value: &str) -> AppResult<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .map_err(infra)?
        .with_timezone(&Utc))
}
