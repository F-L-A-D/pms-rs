use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::domain::reservation::Reservation;

pub async fn create(
    db: &Db,
    inventory: &mut HotelInventory,
    id: String,
    input: StayInput,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let (check_in, check_out) = normalize(input)?;
        let reservation = Reservation::new(id, check_in, check_out)?;

        for d in reservation.nights() {
            inventory.add_reservation(d, 1);
        }

        SqliteReservationRepository::save_tx(&mut tx, &reservation).await?;

        Ok(())
    }.await;

    match result {
        Ok(_) => {
            tx.commit().await.unwrap();
            Ok(())
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            Err(e)
        }
    }
}