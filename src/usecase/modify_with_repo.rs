use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::repository::reservation_repository::ReservationRepository;
use crate::usecase::modify_reservation::modify_reservation;

pub fn modify_with_repo<R: ReservationRepository>(
    repo: &mut R,
    inventory: &mut HotelInventory,
    id: &str,
    input: StayInput,
) -> Result<(), String> {
    let mut reservation = repo
        .find_by_id(id)
        .ok_or("reservation not found")?;

    let (check_in, check_out) = normalize(input)?;

    modify_reservation(
        inventory,
        &mut reservation,
        check_in,
        check_out,
    );

    repo.save(reservation);

    Ok(())
}