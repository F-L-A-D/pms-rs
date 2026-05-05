use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;
use crate::usecase::modify_reservation::modify_reservation;

pub fn modify_from_input(
    inventory: &mut HotelInventory,
    reservation: &mut Reservation,
    input: StayInput,
) -> Result<(), String> {
    let (check_in, check_out) = normalize(input)?;
    
    modify_reservation(
        inventory,
        reservation,
        check_in,
        check_out,
    );

    Ok(())
}