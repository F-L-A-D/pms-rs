#[derive(Debug)]
pub struct InventoryAggregateRow {
    pub room_class: String,
    pub total_rooms: i64,
    pub out_of_order_rooms: i64,
    pub confirmed_reservations: i64,
    pub pending_reservations: i64,
    pub cancelled_reservations: i64,
}
