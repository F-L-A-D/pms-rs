use crate::domain::inventory::HotelInventory;
use crate::repository::reservation_repository::ReservationRepository;
use crate::usecase::cancel_reservation::cancel_reservation;

pub fn cancel_with_repo<R: ReservationRepository>(
    repo: &mut R,
    inventory: &mut HotelInventory,
    id: &str,
) -> Result<(), String> {
    let mut reservation = repo
        .find_by_id(id)
        .ok_or("reservation not found")?;

    cancel_reservation(inventory, &mut reservation);

    repo.save(reservation);

    Ok(())
}