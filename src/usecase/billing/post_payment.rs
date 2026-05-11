use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppError,
        AppResult,
    },

    domain::folio_entry::{
        FolioEntry,
        FolioEntryType,
    },

    repository::sqlite::operational::{
        folio_entry_repository::
            SqliteFolioEntryRepository,

        folio_repository::
            SqliteFolioRepository,
    },
};

pub async fn post_payment(
    db: &Db,

    folio_id: Uuid,

    amount: i64,

    description: String,

) -> AppResult<()> {

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

    let entry =
        FolioEntry::new(
            Uuid::new_v4(),

            folio_id,

            FolioEntryType::Payment,

            amount,

            Some(description),
        );

    SqliteFolioEntryRepository::save(
        &mut tx,
        &entry,
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