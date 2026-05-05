use chrono::NaiveDate;

use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::{Reservation, ReservationStatus};

pub fn modify_reservation(
    inventory: &mut HotelInventory,
    reservation: &mut Reservation,
    new_check_in: NaiveDate,
    new_check_out: NaiveDate,
) {

    if reservation.status == ReservationStatus::Cancelled {
        return;
    }
    
    for date in reservation.nights() {
        inventory.remove_reservation(date, 1);
    }

    if new_check_in > new_check_out {
        return;
    }

    reservation.check_in = new_check_in;
    reservation.check_out = new_check_out;

    for date in reservation.nights() {
        inventory.add_reservation(date, 1);
    }
}