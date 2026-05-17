use crate::{
    api::dto::billing::input::close_folio_input::CloseFolioInput,
    db::connection::Db,
    domain::entity::folio::{Folio, FolioStatus},
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::folio_repository::SqliteFolioRepository,
};

pub async fn execute(db: &Db, input: CloseFolioInput) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut folio = SqliteFolioRepository::find_by_id(&mut tx, input.folio_id)
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        match folio.status {
            FolioStatus::Open | FolioStatus::Locked => {
                folio.status = FolioStatus::Closed;
            }

            FolioStatus::Closed => {
                return Err(conflict("folio already closed"));
            }
        }

        SqliteFolioRepository::save(&mut tx, &folio).await?;

        Ok(folio)
    }
    .await;

    match result {
        Ok(folio) => {
            tx.commit().await.map_err(infra)?;

            Ok(folio)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
