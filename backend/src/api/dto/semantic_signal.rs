use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use serde_json::Value;

use uuid::Uuid;

use crate::projection::signal::model::{
    change_pattern::ChangePattern, confidence_profile::ConfidenceProfile,
    operation_semantic_signal::OperationSemanticSignal, semantic_activation::SemanticActivation,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct OperationSemanticSignalResponse {
    pub event_id: Uuid,
    pub change_pattern: Option<ChangePatternResponse>,
    pub confidence_profile: Option<ConfidenceProfileResponse>,
    pub semantic_activation: Option<SemanticActivationResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePatternResponse {
    pub pattern_type: String,
    pub operation_type: String,
    pub changed_fields: Value,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfidenceProfileResponse {
    pub confidence_score: String,
    pub reasons: Value,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SemanticActivationResponse {
    pub activation_key: String,
    pub activation_score: String,
    pub confidence_score: String,
    pub is_active: bool,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl From<OperationSemanticSignal> for OperationSemanticSignalResponse {
    fn from(signal: OperationSemanticSignal) -> Self {
        Self {
            event_id: signal.event_id,
            change_pattern: signal.change_pattern.map(Into::into),
            confidence_profile: signal.confidence_profile.map(Into::into),
            semantic_activation: signal.semantic_activation.map(Into::into),
        }
    }
}

impl From<ChangePattern> for ChangePatternResponse {
    fn from(pattern: ChangePattern) -> Self {
        Self {
            pattern_type: pattern.pattern_type.to_snake().to_string(),
            operation_type: pattern.operation_type.to_snake().to_string(),
            changed_fields: serde_json::from_str(&pattern.changed_fields_json)
                .unwrap_or(Value::Null),
            projection_version: pattern.projection_version,
            updated_at: pattern.updated_at,
        }
    }
}

impl From<ConfidenceProfile> for ConfidenceProfileResponse {
    fn from(profile: ConfidenceProfile) -> Self {
        Self {
            confidence_score: profile.confidence_score.to_string(),
            reasons: serde_json::from_str(&profile.reasons_json).unwrap_or(Value::Null),
            projection_version: profile.projection_version,
            updated_at: profile.updated_at,
        }
    }
}

impl From<SemanticActivation> for SemanticActivationResponse {
    fn from(activation: SemanticActivation) -> Self {
        Self {
            activation_key: activation.activation_key.to_snake().to_string(),
            activation_score: activation.activation_score.to_string(),
            confidence_score: activation.confidence_score.to_string(),
            is_active: activation.is_active,
            projection_version: activation.projection_version,
            updated_at: activation.updated_at,
        }
    }
}
