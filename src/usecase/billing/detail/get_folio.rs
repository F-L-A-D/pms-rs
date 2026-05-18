use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::folio::Folio,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::folio_repository::SqliteFolioRepository,
};

pub async fn execute(db: &Db, folio_id: Uuid) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = SqliteFolioRepository::find_by_id(&mut tx, folio_id)
        .await?
        .ok_or_else(|| not_found("folio not found"));

    let _ = tx.rollback().await.map_err(infra);

    result
}
