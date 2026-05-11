use uuid::Uuid;

use crate::{
    db::connection::Db,

    repository::sqlite::operational::
        folio_repository::
            SqliteFolioRepository,

    error::app_error::{
        AppError,
        AppResult,
    },
};

pub async fn close_folio(
    db: &Db,
    folio_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await?
        .ok_or(
            AppError::NotFound(
                "folio not found".into()
            )
        )?;

    folio.close()   
        .map_err(
            AppError::Domain
        )?;

    SqliteFolioRepository::save(
        &mut tx,
        &folio,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    Ok(())
}