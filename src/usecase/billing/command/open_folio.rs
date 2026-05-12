use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::folio::Folio,

    error::app_error::{
        AppResult,
        infra,
        not_found,
    },

    repository::sqlite::operational::{
        folio_repository::
            SqliteFolioRepository,

        reservation_repository::
            SqliteReservationRepository,
    },
};

pub async fn open_folio(
    db: &Db,
    reservation_id: Uuid,
) -> AppResult<Folio> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        SqliteReservationRepository
            ::find_by_id(
                &mut tx,
                reservation_id,
            )
            .await?
            .ok_or(
                not_found(
                    "reservation not found"
                )
            )?;

        let folio =
            Folio::new(
                Uuid::new_v4(),
                reservation_id,
                None,
            );

        SqliteFolioRepository
            ::save(
                &mut tx,
                &folio,
            )
            .await?;

        Ok(folio)

    }.await;

    match result {

        Ok(folio) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(folio)
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}