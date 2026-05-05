use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::domain::reservation::ReservationStatus;
use crate::repository::sqlite::inventory_repository::SqliteInventoryRepository;

pub async fn cancel(
    db: &Db,
    id: &str,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let mut res = SqliteReservationRepository::find_by_id_tx(&mut tx, id)
            .await?
            .ok_or("not found")?;

        if res.status == ReservationStatus::Cancelled {
            return Ok(());
        }

        let dates = res.nights();

        for d in dates {
            SqliteInventoryRepository::add_tx(&mut tx, &d.to_string(), -1, 10).await?;
        }

        res.status = ReservationStatus::Cancelled;

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