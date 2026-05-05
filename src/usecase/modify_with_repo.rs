use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::adapter::stay_input::{StayInput, normalize};
use crate::domain::inventory::HotelInventory;
use crate::usecase::modify_reservation::modify_reservation;

pub async fn modify_with_repo(
    db: &Db,
    inventory: &mut HotelInventory,
    id: &str,
    input: StayInput,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let mut res = SqliteReservationRepository::find_by_id_tx(&mut tx, id)
            .await?
            .ok_or("not found")?;

        let (check_in, check_out) = normalize(input)?;

        modify_reservation(inventory, &mut res, check_in, check_out);

        SqliteReservationRepository::save_tx(&mut tx, &res).await?;

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