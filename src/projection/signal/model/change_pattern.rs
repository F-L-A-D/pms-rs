use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::domain::semantic::{
    operation_change_event::OperationType, semantic_activation::ChangePatternType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ChangePattern {
    pub event_id: Uuid,
    pub pattern_type: ChangePatternType,
    pub operation_type: OperationType,
    pub changed_fields_json: String,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}
