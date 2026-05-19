use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_context::{OperationActor, OperationSource},
        operational_audit_log::OperationalAuditLog,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteOperationalAuditLogRepository;

impl SqliteOperationalAuditLogRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        log: &OperationalAuditLog,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO operational_audit_logs (
                id,
                operation_id,
                aggregate_type,
                aggregate_id,
                action,
                actor,
                actor_id,
                source,
                before_json,
                after_json,
                changed_fields_json,
                reason,
                occurred_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            "#,
        )
        .bind(log.id.to_string())
        .bind(log.operation_id.to_string())
        .bind(&log.aggregate_type)
        .bind(log.aggregate_id.to_string())
        .bind(&log.action)
        .bind(log.actor.to_snake())
        .bind(&log.actor_id)
        .bind(log.source.to_snake())
        .bind(&log.before_json)
        .bind(&log.after_json)
        .bind(&log.changed_fields_json)
        .bind(&log.reason)
        .bind(log.occurred_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_aggregate(
        tx: &mut Transaction<'_, Sqlite>,
        aggregate_type: &str,
        aggregate_id: Uuid,
    ) -> AppResult<Vec<OperationalAuditLog>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                operation_id,
                aggregate_type,
                aggregate_id,
                action,
                actor,
                actor_id,
                source,
                before_json,
                after_json,
                changed_fields_json,
                reason,
                occurred_at
            FROM operational_audit_logs
            WHERE aggregate_type = ?1
              AND aggregate_id = ?2
            ORDER BY occurred_at, id
            "#,
        )
        .bind(aggregate_type)
        .bind(aggregate_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter()
            .map(Self::row_to_log)
            .collect::<AppResult<Vec<_>>>()
    }

    fn row_to_log(row: &sqlx::sqlite::SqliteRow) -> AppResult<OperationalAuditLog> {
        let actor = OperationActor::from_snake(row.get::<String, _>("actor").as_str())
            .ok_or_else(|| infra("invalid audit actor"))?;
        let source = OperationSource::from_snake(row.get::<String, _>("source").as_str())
            .ok_or_else(|| infra("invalid audit source"))?;

        Ok(OperationalAuditLog {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            operation_id: Uuid::parse_str(row.get::<String, _>("operation_id").as_str())
                .map_err(infra)?,
            aggregate_type: row.get("aggregate_type"),
            aggregate_id: Uuid::parse_str(row.get::<String, _>("aggregate_id").as_str())
                .map_err(infra)?,
            action: row.get("action"),
            actor,
            actor_id: row.get("actor_id"),
            source,
            before_json: row.get("before_json"),
            after_json: row.get("after_json"),
            changed_fields_json: row.get("changed_fields_json"),
            reason: row.get("reason"),
            occurred_at: parse_datetime(row.get::<String, _>("occurred_at").as_str())?,
        })
    }
}

fn parse_datetime(value: &str) -> AppResult<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .map_err(infra)?
        .with_timezone(&Utc))
}
