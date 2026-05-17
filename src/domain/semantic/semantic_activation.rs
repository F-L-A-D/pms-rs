use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangePatternType {
    ReservationCreated,
    ReservationStayShapeChanged,
    ReservationCancelled,
    ReservationUpdated,
}

impl ChangePatternType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::ReservationCreated => "reservation_created",
            Self::ReservationStayShapeChanged => "reservation_stay_shape_changed",
            Self::ReservationCancelled => "reservation_cancelled",
            Self::ReservationUpdated => "reservation_updated",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "reservation_created" => Some(Self::ReservationCreated),
            "reservation_stay_shape_changed" => Some(Self::ReservationStayShapeChanged),
            "reservation_cancelled" => Some(Self::ReservationCancelled),
            "reservation_updated" => Some(Self::ReservationUpdated),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticActivationKey {
    ReservationCreated,
    StayShapeChanged,
    ReservationCancelled,
    ReservationUpdated,
}

impl SemanticActivationKey {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::ReservationCreated => "reservation_created",
            Self::StayShapeChanged => "stay_shape_changed",
            Self::ReservationCancelled => "reservation_cancelled",
            Self::ReservationUpdated => "reservation_updated",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "reservation_created" => Some(Self::ReservationCreated),
            "stay_shape_changed" => Some(Self::StayShapeChanged),
            "reservation_cancelled" => Some(Self::ReservationCancelled),
            "reservation_updated" => Some(Self::ReservationUpdated),
            _ => None,
        }
    }
}
