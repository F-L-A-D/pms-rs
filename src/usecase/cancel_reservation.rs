use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::{Reservation, ReservationStatus};

pub fn cancel_reservation(
    inventory: &mut HotelInventory,
    reservation: &mut Reservation,
) {

    if reservation.status == ReservationStatus::Cancelled {
        return;
    }

    for date in reservation.nights() {
        inventory.remove_reservation(date, 1);
    }

    reservation.status = ReservationStatus::Cancelled;
}