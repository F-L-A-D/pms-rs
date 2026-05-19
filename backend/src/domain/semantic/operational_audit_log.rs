use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::domain::semantic::operation_context::{OperationActor, OperationSource};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalAuditLog {
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
