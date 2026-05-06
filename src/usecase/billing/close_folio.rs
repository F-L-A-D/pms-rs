use crate::db::connection::Db;

use crate::repository::sqlite::folio_repository::SqliteFolioRepository;

pub async fn close_folio(
    db: &Db,
    folio_id: &str,
) -> Result<(), String> {

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &db.pool,
            folio_id,
        )
        .await?
        .ok_or("folio not found")?;

    folio.close()?;

    SqliteFolioRepository::save(
        &db.pool,
        &folio,
    )
    .await?;

    Ok(())
}