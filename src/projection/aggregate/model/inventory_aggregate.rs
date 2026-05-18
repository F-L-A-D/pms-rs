use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct InventoryAggregate {
    pub service_date: NaiveDate,
    pub room_class: String,
    pub total_rooms: i64,
    pub out_of_order_rooms: i64,
    pub reservable_rooms: i64,
    pub confirmed_reservations: i64,
    pub pending_reservations: i64,
    pub cancelled_reservations: i64,
    pub available_rooms: i64,
    pub available_rooms_including_pending: i64,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for InventoryAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.service_date == other.service_date
            && self.room_class == other.room_class
            && self.total_rooms == other.total_rooms
            && self.out_of_order_rooms == other.out_of_order_rooms
            && self.reservable_rooms == other.reservable_rooms
            && self.confirmed_reservations == other.confirmed_reservations
            && self.pending_reservations == other.pending_reservations
            && self.cancelled_reservations == other.cancelled_reservations
            && self.available_rooms == other.available_rooms
            && self.available_rooms_including_pending == other.available_rooms_including_pending
            && self.projection_version == other.projection_version
    }
}
