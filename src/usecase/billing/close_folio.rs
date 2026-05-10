use uuid::Uuid;

use crate::{
    db::connection::Db,

    repository::sqlite::operational::
        folio_repository::
            SqliteFolioRepository,
};

pub async fn close_folio(
    db: &Db,
    folio_id: Uuid,
) -> Result<(), String> {

    let mut tx =
        db.begin_tx().await;

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await?
        .ok_or(
            "folio not found"
        )?;

    folio.close()?;

    SqliteFolioRepository::save(
        &mut tx,
        &folio,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}