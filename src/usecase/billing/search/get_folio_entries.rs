use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::folio_entry::FolioEntry,

    error::app_error::{
        AppResult,
        not_found,
    },

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

    SqliteFolioRepository
        ::find_by_id(
            &mut tx,
            folio_id,
        )
        .await?
        .ok_or(
            not_found(
                "folio not found"
            )
        )?;

    let entries =
        SqliteFolioEntryRepository
            ::find_by_folio_id(
                &mut tx,
                folio_id,
            )
            .await?;

    let _ =
        tx.rollback().await;

    Ok(entries)
}