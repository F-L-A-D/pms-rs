use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;

pub fn create_reservation(
    inventory: &mut HotelInventory,
    reservation: &Reservation,
) {
    for date in reservation.nights() {
        inventory.add_reservation(date, 1);
    }
}
