use crate::db::connection::Db;

use crate::domain::folio::Folio;

use crate::repository::sqlite::folio_repository::SqliteFolioRepository;
use crate::repository::sqlite::repository::SqliteReservationRepository;

pub async fn open_folio(
    db: &Db,
    folio_id: String,
    reservation_id: String,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {

        let reservation =
            SqliteReservationRepository::find_by_id_tx(
                &mut tx,
                &reservation_id,
            )
            .await?;

        if reservation.is_none() {
            return Err("reservation not found".into());
        }

        let folio = Folio::new(
            folio_id,
            reservation_id,
        );

        SqliteFolioRepository::save(
            &db.pool,
            &folio,
        )
        .await?;

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