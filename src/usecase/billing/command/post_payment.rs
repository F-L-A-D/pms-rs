use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::folio_entry::{
        FolioEntry,
        FolioEntryType,
    },

    error::app_error::{
        AppResult,
        infra,
        not_found,
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

    let result = async {

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

        let entry =
            FolioEntry::new(
                Uuid::new_v4(),

                folio_id,

                FolioEntryType::Payment,

                amount,

                Some(description),
            );

        SqliteFolioEntryRepository
            ::save(
                &mut tx,
                &entry,
            )
            .await?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}