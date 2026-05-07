use crate::db::connection::Db;

use crate::domain::folio_entry::{
    EntryType,
    FolioEntry,
};

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;
use crate::repository::sqlite::folio_repository::SqliteFolioRepository;

pub async fn post_room_charge(
    db: &Db,
    entry_id: String,
    folio_id: &str,
    amount: i64,
    description: Option<String>,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let folio =
            SqliteFolioRepository::find_by_id(
                &mut tx,
                folio_id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        if folio.is_none() {

            return Err(
                AppError::NotFound(
                    "folio not found".into()
                )
            );
        }

        let entry =
            FolioEntry::new(
                entry_id,
                folio_id.into(),
                EntryType::RoomCharge,
                amount,
                description,
            );

        SqliteFolioEntryRepository::save(
            &mut tx,
            &entry,
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