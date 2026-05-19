#[derive(Debug)]
pub struct HousekeepingDailyWorkloadAggregateRow {
    pub room_class: String,
    pub total_tracked_rooms: i64,
    pub dirty_rooms: i64,
    pub cleaning_rooms: i64,
    pub cleaned_rooms: i64,
    pub inspected_rooms: i64,
    pub occupied_rooms: i64,
    pub vacant_rooms: i64,
    pub out_of_order_rooms: i64,
}
