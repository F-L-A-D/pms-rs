#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct HotelInventoryProjection {
    pub date: String,
    pub reserved_rooms: i32,
}