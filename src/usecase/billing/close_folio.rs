use uuid::Uuid;

use crate::db::connection::Db;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::
    folio_repository::SqliteFolioRepository;

pub async fn close_folio(
    db: &Db,
    folio_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut folio =
            SqliteFolioRepository::find_by_id(
                &mut tx,
                folio_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "folio not found".into()
                )
            )?;

        folio.close()
            .map_err(AppError::Conflict)?;

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