use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationTraceKind {
    DepartmentTrace,
}

impl ReservationTraceKind {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::DepartmentTrace => "department_trace",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "department_trace" => Some(Self::DepartmentTrace),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationTrace {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub kind: ReservationTraceKind,
    pub department_code: Option<String>,
    pub body: String,
    pub actor_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}
