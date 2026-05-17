use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationActor {
    System,
    Staff,
    Guest,
    ExternalSystem,
}

impl OperationActor {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Staff => "staff",
            Self::Guest => "guest",
            Self::ExternalSystem => "external_system",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "system" => Some(Self::System),
            "staff" => Some(Self::Staff),
            "guest" => Some(Self::Guest),
            "external_system" => Some(Self::ExternalSystem),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationSource {
    Api,
    Batch,
    Import,
    Internal,
}

impl OperationSource {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Batch => "batch",
            Self::Import => "import",
            Self::Internal => "internal",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "api" => Some(Self::Api),
            "batch" => Some(Self::Batch),
            "import" => Some(Self::Import),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationContext {
    pub operation_id: Uuid,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
}

impl OperationContext {
    pub fn api_system() -> Self {
        Self {
            operation_id: Uuid::new_v4(),
            actor: OperationActor::System,
            actor_id: None,
            source: OperationSource::Api,
        }
    }
}
