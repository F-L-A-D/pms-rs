use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::semantic::{
    operation_context::{OperationActor, OperationSource},
    operational_audit_log::OperationalAuditLog,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalAuditLogResponse {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub action: String,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl From<OperationalAuditLog> for OperationalAuditLogResponse {
    fn from(log: OperationalAuditLog) -> Self {
        Self {
            id: log.id,
            operation_id: log.operation_id,
            aggregate_type: log.aggregate_type,
            aggregate_id: log.aggregate_id,
            action: log.action,
            actor: log.actor,
            actor_id: log.actor_id,
            source: log.source,
            before_json: log.before_json,
            after_json: log.after_json,
            changed_fields_json: log.changed_fields_json,
            reason: log.reason,
            occurred_at: log.occurred_at,
        }
    }
}
