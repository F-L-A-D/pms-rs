use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FolioStatus {
    Open,
    Locked,
    Closed,
}

impl FolioStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Locked => "locked",
            Self::Closed => "closed",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "open" => Some(Self::Open),
            "locked" => Some(Self::Locked),
            "closed" => Some(Self::Closed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folio {
    pub id: Uuid,

    pub reservation_id: Uuid,

    pub billing_account_id: Option<Uuid>,

    pub status: FolioStatus,

    pub created_at: DateTime<Utc>,
}
