use uuid::Uuid;

use super::{
    change_pattern::ChangePattern, confidence_profile::ConfidenceProfile,
    semantic_activation::SemanticActivation,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OperationSemanticSignal {
    pub event_id: Uuid,
    pub change_pattern: Option<ChangePattern>,
    pub confidence_profile: Option<ConfidenceProfile>,
    pub semantic_activation: Option<SemanticActivation>,
}
