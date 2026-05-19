use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct HousekeepingDailyWorkloadAggregate {
    pub service_date: NaiveDate,
    pub room_class: String,
    pub total_tracked_rooms: i64,
    pub dirty_rooms: i64,
    pub cleaning_rooms: i64,
    pub cleaned_rooms: i64,
    pub inspected_rooms: i64,
    pub occupied_rooms: i64,
    pub vacant_rooms: i64,
    pub out_of_order_rooms: i64,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for HousekeepingDailyWorkloadAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.service_date == other.service_date
            && self.room_class == other.room_class
            && self.total_tracked_rooms == other.total_tracked_rooms
            && self.dirty_rooms == other.dirty_rooms
            && self.cleaning_rooms == other.cleaning_rooms
            && self.cleaned_rooms == other.cleaned_rooms
            && self.inspected_rooms == other.inspected_rooms
            && self.occupied_rooms == other.occupied_rooms
            && self.vacant_rooms == other.vacant_rooms
            && self.out_of_order_rooms == other.out_of_order_rooms
            && self.projection_version == other.projection_version
    }
}
