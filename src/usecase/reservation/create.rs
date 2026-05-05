use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::reservation::Reservation;
use crate::repository::sqlite::inventory_repository::SqliteInventoryRepository;

pub async fn create(
    db: &Db,
    id: String,
    input: StayInput,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let (check_in, check_out) = normalize(input)?;
        let reservation = Reservation::new(id, check_in, check_out)?;

        SqliteReservationRepository::save_tx(&mut tx, &reservation).await?;

        for d in reservation.nights() {
            SqliteInventoryRepository::add_tx(
                &mut tx,
                &d.to_string(),
                1,
                10,
            ).await?;
        }

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