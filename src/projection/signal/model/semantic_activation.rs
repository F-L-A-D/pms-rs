use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::domain::semantic::semantic_activation::SemanticActivationKey;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticActivation {
    pub event_id: Uuid,
    pub activation_key: SemanticActivationKey,
    pub activation_score: Decimal,
    pub confidence_score: Decimal,
    pub is_active: bool,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}
