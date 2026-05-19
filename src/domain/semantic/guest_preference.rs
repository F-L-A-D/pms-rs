use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuestPreferenceType {
    Room,
    Pillow,
    Dietary,
    Accessibility,
    Communication,
    Other,
}

impl GuestPreferenceType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Room => "room",
            Self::Pillow => "pillow",
            Self::Dietary => "dietary",
            Self::Accessibility => "accessibility",
            Self::Communication => "communication",
            Self::Other => "other",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "room" => Some(Self::Room),
            "pillow" => Some(Self::Pillow),
            "dietary" => Some(Self::Dietary),
            "accessibility" => Some(Self::Accessibility),
            "communication" => Some(Self::Communication),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuestPreference {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub preference_type: GuestPreferenceType,
    pub value: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
