use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;
use crate::transaction::transaction::run_in_transaction;

pub fn create_with_repo<R: ReservationRepository + Clone>(
    repo: &mut R,
    inventory: &mut HotelInventory,
    id: String,
    input: StayInput,
) -> Result<(), String> {

    run_in_transaction(repo, inventory, |repo, inventory| {
        let (check_in, check_out) = normalize(input)?;

        let reservation = Reservation::new(id.clone(), check_in, check_out)?;

        for date in reservation.nights() {
            inventory.add_reservation(date, 1);
        }

        repo.save(reservation);

        Ok(())
    })
}