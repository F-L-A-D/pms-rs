use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_trace::{ReservationTrace, ReservationTraceKind},
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationTraceRepository;

impl SqliteReservationTraceRepository {
    pub async fn save(tx: &mut Transaction<'_, Sqlite>, trace: &ReservationTrace) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_traces (
                id,
                reservation_id,
                kind,
                department_code,
                body,
                actor_id,
                created_at,
                resolved_at,
                resolved_by,
                deleted_at,
                deleted_by
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
        )
        .bind(trace.id.to_string())
        .bind(trace.reservation_id.to_string())
        .bind(trace.kind.to_snake())
        .bind(&trace.department_code)
        .bind(&trace.body)
        .bind(&trace.actor_id)
        .bind(trace.created_at.to_rfc3339())
        .bind(trace.resolved_at.map(|value| value.to_rfc3339()))
        .bind(&trace.resolved_by)
        .bind(trace.deleted_at.map(|value| value.to_rfc3339()))
        .bind(&trace.deleted_by)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationTrace>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                kind,
                department_code,
                body,
                actor_id,
                created_at,
                resolved_at,
                resolved_by,
                deleted_at,
                deleted_by
            FROM reservation_traces
            WHERE reservation_id = ?1
              AND deleted_at IS NULL
            ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_trace).collect()
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        trace_id: Uuid,
    ) -> AppResult<Option<ReservationTrace>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                kind,
                department_code,
                body,
                actor_id,
                created_at,
                resolved_at,
                resolved_by,
                deleted_at,
                deleted_by
            FROM reservation_traces
            WHERE id = ?1
            "#,
        )
        .bind(trace_id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(|row| Self::row_to_trace(&row)).transpose()
    }

    pub async fn mark_resolved(
        tx: &mut Transaction<'_, Sqlite>,
        trace_id: Uuid,
        resolved_at: DateTime<Utc>,
        resolved_by: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE reservation_traces
            SET
                resolved_at = ?1,
                resolved_by = ?2
            WHERE id = ?3
              AND deleted_at IS NULL
            "#,
        )
        .bind(resolved_at.to_rfc3339())
        .bind(resolved_by)
        .bind(trace_id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn mark_deleted(
        tx: &mut Transaction<'_, Sqlite>,
        trace_id: Uuid,
        deleted_at: DateTime<Utc>,
        deleted_by: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE reservation_traces
            SET
                deleted_at = ?1,
                deleted_by = ?2
            WHERE id = ?3
              AND deleted_at IS NULL
            "#,
        )
        .bind(deleted_at.to_rfc3339())
        .bind(deleted_by)
        .bind(trace_id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    fn row_to_trace(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationTrace> {
        let kind = ReservationTraceKind::from_snake(row.get::<String, _>("kind").as_str())
            .ok_or_else(|| infra("invalid reservation trace kind"))?;

        Ok(ReservationTrace {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            kind,
            department_code: row.get("department_code"),
            body: row.get("body"),
            actor_id: row.get("actor_id"),
            created_at: parse_datetime(row.get::<String, _>("created_at").as_str())?,
            resolved_at: row
                .get::<Option<String>, _>("resolved_at")
                .map(|value| parse_datetime(value.as_str()))
                .transpose()?,
            resolved_by: row.get("resolved_by"),
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
