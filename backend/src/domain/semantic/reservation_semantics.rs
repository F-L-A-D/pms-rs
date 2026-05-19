use crate::domain::{
    entity::reservation::Reservation, semantic::guest_timeline_event::TimelineEventType,
};

use crate::domain::semantic::reservation_transition::{
    ReservationTransitionChange, ReservationTransitionType,
};

pub fn detect_reservation_timeline_events(
    before: &Reservation,
    after: &Reservation,
) -> Vec<TimelineEventType> {
    let mut events = vec![];

    if before.check_in != after.check_in || before.check_out != after.check_out {
        events.push(TimelineEventType::ReservationDatesChanged);
    }

    if after.check_out > before.check_out {
        events.push(TimelineEventType::ReservationExtended);
    }

    if after.check_out < before.check_out {
        events.push(TimelineEventType::ReservationShortened);
    }

    if before.room_class != after.room_class {
        events.push(TimelineEventType::ReservationRoomClassChanged);
    }

    if !events.is_empty() {
        events.push(TimelineEventType::ReservationModified);
    }

    events
}

pub fn detect_reservation_transition_changes(
    before: &Reservation,
    after: &Reservation,
) -> Vec<ReservationTransitionChange> {
    let mut changes = vec![];

    if before.check_in != after.check_in {
        changes.push(ReservationTransitionChange {
            transition_type: ReservationTransitionType::CheckInChanged,
            field_name: "check_in",
            before_value: before.check_in.to_string(),
            after_value: after.check_in.to_string(),
        });
    }

    if before.check_out != after.check_out {
        changes.push(ReservationTransitionChange {
            transition_type: ReservationTransitionType::CheckOutChanged,
            field_name: "check_out",
            before_value: before.check_out.to_string(),
            after_value: after.check_out.to_string(),
        });
    }

    if after.check_out > before.check_out {
        changes.push(ReservationTransitionChange {
            transition_type: ReservationTransitionType::ReservationExtended,
            field_name: "check_out",
            before_value: before.check_out.to_string(),
            after_value: after.check_out.to_string(),
        });
    }

    if after.check_out < before.check_out {
        changes.push(ReservationTransitionChange {
            transition_type: ReservationTransitionType::ReservationShortened,
            field_name: "check_out",
            before_value: before.check_out.to_string(),
            after_value: after.check_out.to_string(),
        });
    }

    if before.room_class != after.room_class {
        changes.push(ReservationTransitionChange {
            transition_type: ReservationTransitionType::RoomClassChanged,
            field_name: "room_class",
            before_value: before.room_class.clone(),
            after_value: after.room_class.clone(),
        });
    }

    changes
}
