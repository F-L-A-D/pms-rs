use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum TimelineEventType {
    ReservationCreated,
    ReservationCancelled,
    ReservationNoShow,
    ReservationReinstated,

    ReservationModified,

    ReservationDatesChanged,
    ReservationExtended,
    ReservationShortened,

    ReservationRoomClassChanged,

    CheckedIn,
    CheckedOut,
    RoomMoved,

    RoomChargePosted,
}

impl TimelineEventType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::ReservationCreated => "reservation_created",
            Self::ReservationCancelled => "reservation_cancelled",
            Self::ReservationNoShow => "reservation_no_show",
            Self::ReservationReinstated => "reservation_reinstated",
            Self::ReservationModified => "reservation_modified",
            Self::ReservationDatesChanged => "reservation_dates_changed",
            Self::ReservationExtended => "reservation_extended",
            Self::ReservationShortened => "reservation_shortened",
            Self::ReservationRoomClassChanged => "reservation_room_class_changed",
            Self::CheckedIn => "checked_in",
            Self::CheckedOut => "checked_out",
            Self::RoomMoved => "room_moved",
            Self::RoomChargePosted => "room_charge_posted",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "reservation_created" => Some(Self::ReservationCreated),
            "reservation_cancelled" => Some(Self::ReservationCancelled),
            "reservation_no_show" => Some(Self::ReservationNoShow),
            "reservation_reinstated" => Some(Self::ReservationReinstated),
            "reservation_modified" => Some(Self::ReservationModified),
            "reservation_dates_changed" => Some(Self::ReservationDatesChanged),
            "reservation_extended" => Some(Self::ReservationExtended),
            "reservation_shortened" => Some(Self::ReservationShortened),
            "reservation_room_class_changed" => Some(Self::ReservationRoomClassChanged),
            "checked_in" => Some(Self::CheckedIn),
            "checked_out" => Some(Self::CheckedOut),
            "room_moved" => Some(Self::RoomMoved),
            "room_charge_posted" => Some(Self::RoomChargePosted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuestTimelineEvent {
    pub id: Uuid,
    pub guest_id: Uuid,
    pub event_type: TimelineEventType,
    pub reference_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl GuestTimelineEvent {
    pub fn new(
        id: Uuid,
        guest_id: Uuid,
        event_type: TimelineEventType,
        reference_id: Uuid,
    ) -> Self {
        Self {
            id,
            guest_id,
            event_type,
            reference_id,
            occurred_at: Utc::now(),
        }
    }
}
