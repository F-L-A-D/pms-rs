use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;

pub fn create_reservation(
    inventory: &mut HotelInventory,
    reservation: Reservation,
) {
    inventory.add_reservation(1);
    let _ = reservation;
}
