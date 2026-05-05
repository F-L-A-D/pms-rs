use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::adapter::stay_input::{StayInput, normalize};
use crate::repository::sqlite::inventory_repository::SqliteInventoryRepository;

pub async fn modify(
    db: &Db,
    id: &str,
    input: StayInput,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {
        let mut res = SqliteReservationRepository::find_by_id_tx(&mut tx, id)
            .await?
            .ok_or("not found")?;

        let old_dates = res.nights();

        let (check_in, check_out) = normalize(input)?;

        res.check_in = check_in;
        res.check_out = check_out;

        let new_dates = res.nights();

        for d in old_dates {
            SqliteInventoryRepository::add_tx(&mut tx, &d.to_string(), -1, 10).await?;
        }

        for d in new_dates {
            SqliteInventoryRepository::add_tx(&mut tx, &d.to_string(), 1, 10).await?;
        }

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