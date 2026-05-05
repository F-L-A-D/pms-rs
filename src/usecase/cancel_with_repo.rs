use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::domain::inventory::HotelInventory;
use crate::usecase::cancel_reservation::cancel_reservation;

pub async fn cancel_with_repo(
    db: &Db,
    inventory: &mut HotelInventory,
    id: &str,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let mut res = SqliteReservationRepository::find_by_id_tx(&mut tx, id)
            .await?
            .ok_or("not found")?;

        cancel_reservation(inventory, &mut res);

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