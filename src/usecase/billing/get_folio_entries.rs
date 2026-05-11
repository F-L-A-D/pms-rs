use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppError,
        AppResult,
    },

    domain::folio_entry::FolioEntry,

    repository::sqlite::operational::{
        folio_entry_repository::
            SqliteFolioEntryRepository,

        folio_repository::
            SqliteFolioRepository,
    },
};

pub async fn get_folio_entries(
    db: &Db,
    folio_id: Uuid,

) -> AppResult<Vec<FolioEntry>> {

    let mut tx =
        db.begin_tx().await;

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

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            &mut tx,
            folio_id,
        )
        .await?;

    Ok(entries)
}