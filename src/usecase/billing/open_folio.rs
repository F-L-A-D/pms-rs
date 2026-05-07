use crate::db::connection::Db;

use crate::domain::folio::Folio;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::folio_repository::SqliteFolioRepository;
use crate::repository::sqlite::reservation_repository::SqliteReservationRepository;

pub async fn open_folio(
    db: &Db,
    folio_id: String,
    reservation_id: String,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let reservation =
            SqliteReservationRepository::find_by_id(
                &mut tx,
                &reservation_id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        if reservation.is_none() {

            return Err(
                AppError::NotFound(
                    "reservation not found".into()
                )
            );
        }

        let folio =
            Folio::new(
                folio_id,
                reservation_id,
            );

        SqliteFolioRepository::save(
            &mut tx,
            &folio,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Ok(())
        }

        Err(e) => {

            tx.rollback()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Err(e)
        }
    }
}