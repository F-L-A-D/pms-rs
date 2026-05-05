use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;
use crate::usecase::create_reservation::create_reservation;

pub fn create_from_input(
    inventory: &mut HotelInventory,
    id: String,
    input: StayInput,
) -> Result<Reservation, String> {
    let (check_in, check_out) = normalize(input)?;
    
    let reservation = Reservation::new(id, check_in, check_out)?;
    
    create_reservation(inventory, &reservation);
    
    Ok(reservation)
}