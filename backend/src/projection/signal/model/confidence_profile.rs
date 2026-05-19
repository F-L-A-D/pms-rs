use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceProfile {
    pub event_id: Uuid,
    pub confidence_score: Decimal,
    pub reasons_json: String,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}
