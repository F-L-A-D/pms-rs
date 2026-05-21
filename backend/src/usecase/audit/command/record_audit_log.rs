use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_context::OperationContext, operational_audit_log::OperationalAuditLog,
    },
    error::app_error::AppResult,
    repository::sqlite::operational::operation::operational_audit_log_repository::SqliteOperationalAuditLogRepository,
};

pub struct RecordAuditLogInput {
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub action: String,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,
    pub reason: Option<String>,
}

pub async fn record_audit_log(
    tx: &mut Transaction<'_, Sqlite>,
    context: &OperationContext,
    input: RecordAuditLogInput,
) -> AppResult<OperationalAuditLog> {
    let log = OperationalAuditLog {
        id: Uuid::new_v4(),
        operation_id: context.operation_id,
        aggregate_type: input.aggregate_type,
        aggregate_id: input.aggregate_id,
        action: input.action,
        actor: context.actor,
        actor_id: context.actor_id.clone(),
        source: context.source,
        before_json: input.before_json,
        after_json: input.after_json,
        changed_fields_json: input.changed_fields_json,
        reason: input.reason,
        occurred_at: chrono::Utc::now(),
    };

    SqliteOperationalAuditLogRepository::save(tx, &log).await?;

    Ok(log)
}
