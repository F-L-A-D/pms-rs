use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationGuestRelationType {
    Primary,
    Accompany,
}

impl ReservationGuestRelationType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Accompany => "accompany",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "primary" => Some(Self::Primary),
            "accompany" => Some(Self::Accompany),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationGuestRelation {
    pub reservation_id: Uuid,
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

impl ReservationGuestRelation {
    pub fn is_primary(&self) -> bool {
        self.relation_type == ReservationGuestRelationType::Primary
    }
}
