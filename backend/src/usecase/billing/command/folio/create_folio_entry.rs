use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::create_folio_entry_input::CreateFolioEntryInput,
    db::connection::Db,
    domain::entity::{folio::FolioStatus, folio_entry::FolioEntry},
    error::app_error::{domain, infra, not_found, AppResult},
    repository::sqlite::operational::billing::{
        folio_entry_repository::SqliteFolioEntryRepository, folio_repository::SqliteFolioRepository,
    },
};

pub async fn execute(db: &Db, input: CreateFolioEntryInput) -> AppResult<FolioEntry> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = match SqliteFolioRepository::find_by_id(&mut tx, input.folio_id).await? {
            Some(folio) => folio,

            None => {
                return Err(not_found("folio not found"));
            }
        };

        if !matches!(folio.status, FolioStatus::Open) {
            return Err(domain("folio is not open"));
        }

        let entry = FolioEntry {
            id: Uuid::new_v4(),

            folio_id: input.folio_id,

            entry_type: input.entry_type,

            amount: input.amount,

            occurred_at: Utc::now(),

            memo: input.memo,
        };

        SqliteFolioEntryRepository::save(&mut tx, &entry).await?;

        Ok(entry)
    }
    .await;

    match result {
        Ok(entry) => {
            tx.commit().await.map_err(infra)?;

            Ok(entry)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
