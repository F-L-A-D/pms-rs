use crate::domain::{
    entity::reservation::Reservation, semantic::guest_timeline_event::TimelineEventType,
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
