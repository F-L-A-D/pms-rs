use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationTransitionType {
    CheckInChanged,
    CheckOutChanged,
    ReservationExtended,
    ReservationShortened,
    RoomClassChanged,
}

impl ReservationTransitionType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::CheckInChanged => "check_in_changed",
            Self::CheckOutChanged => "check_out_changed",
            Self::ReservationExtended => "reservation_extended",
            Self::ReservationShortened => "reservation_shortened",
            Self::RoomClassChanged => "room_class_changed",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "check_in_changed" => Some(Self::CheckInChanged),
            "check_out_changed" => Some(Self::CheckOutChanged),
            "reservation_extended" => Some(Self::ReservationExtended),
            "reservation_shortened" => Some(Self::ReservationShortened),
            "room_class_changed" => Some(Self::RoomClassChanged),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservationTransitionChange {
    pub transition_type: ReservationTransitionType,
    pub field_name: &'static str,
    pub before_value: String,
    pub after_value: String,
}

impl ReservationTransitionChange {
    pub fn into_transition(self, reservation_id: Uuid) -> ReservationTransition {
        ReservationTransition {
            id: Uuid::new_v4(),
            reservation_id,
            transition_type: self.transition_type,
            field_name: self.field_name.to_string(),
            before_value: self.before_value,
            after_value: self.after_value,
            occurred_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservationTransition {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub transition_type: ReservationTransitionType,
    pub field_name: String,
    pub before_value: String,
    pub after_value: String,
    pub occurred_at: DateTime<Utc>,
}
