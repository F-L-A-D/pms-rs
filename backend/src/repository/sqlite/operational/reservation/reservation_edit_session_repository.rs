use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_edit_session::ReservationEditSession,
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationEditSessionRepository;

impl SqliteReservationEditSessionRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        session: &ReservationEditSession,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_edit_sessions (
                id,
                reservation_id,
                actor_id,
                actor_label,
                opened_at,
                expires_at,
                closed_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(session.id.to_string())
        .bind(session.reservation_id.to_string())
        .bind(&session.actor_id)
        .bind(&session.actor_label)
        .bind(session.opened_at.to_rfc3339())
        .bind(session.expires_at.to_rfc3339())
        .bind(session.closed_at.map(|value| value.to_rfc3339()))
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_active_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
        now: DateTime<Utc>,
    ) -> AppResult<Vec<ReservationEditSession>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                actor_id,
                actor_label,
                opened_at,
                expires_at,
                closed_at
            FROM reservation_edit_sessions
            WHERE reservation_id = ?1
              AND closed_at IS NULL
              AND expires_at > ?2
            ORDER BY opened_at
            "#,
        )
        .bind(reservation_id.to_string())
        .bind(now.to_rfc3339())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter()
            .map(Self::row_to_session)
            .collect::<AppResult<Vec<_>>>()
    }

    pub async fn close(
        tx: &mut Transaction<'_, Sqlite>,
        session_id: Uuid,
        actor_id: &str,
        closed_at: DateTime<Utc>,
    ) -> AppResult<bool> {
        let result = sqlx::query(
            r#"
            UPDATE reservation_edit_sessions
            SET closed_at = ?1
            WHERE id = ?2
              AND actor_id = ?3
              AND closed_at IS NULL
            "#,
        )
        .bind(closed_at.to_rfc3339())
        .bind(session_id.to_string())
        .bind(actor_id)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_session(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationEditSession> {
        Ok(ReservationEditSession {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            actor_id: row.get("actor_id"),
            actor_label: row.get("actor_label"),
            opened_at: parse_datetime(row.get::<String, _>("opened_at").as_str())?,
            expires_at: parse_datetime(row.get::<String, _>("expires_at").as_str())?,
            closed_at: row
                .get::<Option<String>, _>("closed_at")
                .map(|value| parse_datetime(value.as_str()))
                .transpose()?,
        })
    }
}

fn parse_datetime(value: &str) -> AppResult<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .map_err(infra)?
        .with_timezone(&Utc))
}
