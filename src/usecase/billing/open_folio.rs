use crate::db::connection::Db;

use uuid::Uuid;

use crate::domain::folio::Folio;

use crate::error::app_error::{AppError, AppResult};

use crate::repository::sqlite::operational::{
    folio_repository::SqliteFolioRepository, reservation_repository::SqliteReservationRepository,
};

pub async fn open_folio(db: &Db, reservation_id: Uuid) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await
            .map_err(AppError::Infrastructure)?;

        if reservation.is_none() {
            return Err(AppError::NotFound("reservation not found".into()));
        }

        let folio = Folio::new(Uuid::new_v4(), reservation_id);

        SqliteFolioRepository::save(&mut tx, &folio)
            .await
            .map_err(AppError::Infrastructure)?;

        Ok(folio)
    }
    .await;

    match result {
        Ok(folio) => {
            tx.commit()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Ok(folio)
        }

        Err(e) => {
            tx.rollback()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Err(e)
        }
    }
}
