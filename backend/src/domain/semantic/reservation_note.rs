use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationNoteKind {
    GlobalMemo,
    DepartmentTrace,
}

impl ReservationNoteKind {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::GlobalMemo => "global_memo",
            Self::DepartmentTrace => "department_trace",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "global_memo" => Some(Self::GlobalMemo),
            "department_trace" => Some(Self::DepartmentTrace),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationNote {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub kind: ReservationNoteKind,
    pub department_code: Option<String>,
    pub body: String,
    pub actor_id: Option<String>,
    pub created_at: DateTime<Utc>,
}
