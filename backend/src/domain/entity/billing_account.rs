use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingAccountStatus {
    Active,
    Suspended,
}

impl BillingAccountStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "active" => Some(Self::Active),
            "suspended" => Some(Self::Suspended),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAccount {
    pub id: Uuid,

    pub company_id: Option<Uuid>,

    pub name: String,

    pub status: BillingAccountStatus,

    pub created_at: DateTime<Utc>,
}
