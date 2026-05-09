#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionNode {
    GuestSummary,
    ReservationSearch,
    Inventory,
    HotelInventory,
}