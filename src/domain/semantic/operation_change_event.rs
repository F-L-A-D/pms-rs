use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use super::operation_context::{OperationActor, OperationSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Create,
    Modify,
    Cancel,
}

impl OperationType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Modify => "modify",
            Self::Cancel => "cancel",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "create" => Some(Self::Create),
            "modify" => Some(Self::Modify),
            "cancel" => Some(Self::Cancel),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationChangeEvent {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub operation_type: OperationType,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,
    pub occurred_at: DateTime<Utc>,
}
