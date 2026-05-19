use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::{OperationChangeEvent, OperationType},
        operation_context::{OperationActor, OperationSource},
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteOperationChangeEventRepository;

impl SqliteOperationChangeEventRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        event: &OperationChangeEvent,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO operation_change_events (
                id,
                operation_id,
                aggregate_type,
                aggregate_id,
                operation_type,
                actor,
                actor_id,
                source,
                before_json,
                after_json,
                changed_fields_json,
                occurred_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            "#,
        )
        .bind(event.id.to_string())
        .bind(event.operation_id.to_string())
        .bind(&event.aggregate_type)
        .bind(event.aggregate_id.to_string())
        .bind(event.operation_type.to_snake())
        .bind(event.actor.to_snake())
        .bind(&event.actor_id)
        .bind(event.source.to_snake())
        .bind(&event.before_json)
        .bind(&event.after_json)
        .bind(&event.changed_fields_json)
        .bind(event.occurred_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<OperationChangeEvent>> {
        let row = sqlx::query(
            r#"
            SELECT *
            FROM operation_change_events
            WHERE id = ?1
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        row.map(|row| Self::row_to_event(&row)).transpose()
    }

    pub async fn list_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Vec<OperationChangeEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT *
            FROM operation_change_events
            ORDER BY occurred_at, id
            "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_event).collect()
    }

    pub async fn list_by_aggregate(
        tx: &mut Transaction<'_, Sqlite>,
        aggregate_type: &str,
        aggregate_id: Uuid,
    ) -> AppResult<Vec<OperationChangeEvent>> {
        let rows = sqlx::query(
            r#"
            SELECT *
            FROM operation_change_events
            WHERE aggregate_type = ?1
              AND aggregate_id = ?2
            ORDER BY occurred_at DESC, id DESC
            "#,
        )
        .bind(aggregate_type)
        .bind(aggregate_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_event).collect()
    }

    fn row_to_event(row: &sqlx::sqlite::SqliteRow) -> AppResult<OperationChangeEvent> {
        Ok(OperationChangeEvent {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            operation_id: Uuid::parse_str(row.get::<String, _>("operation_id").as_str())
                .map_err(infra)?,
            aggregate_type: row.get("aggregate_type"),
            aggregate_id: Uuid::parse_str(row.get::<String, _>("aggregate_id").as_str())
                .map_err(infra)?,
            operation_type: OperationType::from_snake(
                row.get::<String, _>("operation_type").as_str(),
            )
            .ok_or_else(|| infra("invalid operation type"))?,
            actor: OperationActor::from_snake(row.get::<String, _>("actor").as_str())
                .ok_or_else(|| infra("invalid operation actor"))?,
            actor_id: row.get("actor_id"),
            source: OperationSource::from_snake(row.get::<String, _>("source").as_str())
                .ok_or_else(|| infra("invalid operation source"))?,
            before_json: row.get("before_json"),
            after_json: row.get("after_json"),
            changed_fields_json: row.get("changed_fields_json"),
            occurred_at: row
                .get::<String, _>("occurred_at")
                .parse::<DateTime<Utc>>()
                .map_err(infra)?,
        })
    }
}
