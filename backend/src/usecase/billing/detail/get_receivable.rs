use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::receivable::Receivable,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::billing::receivable_repository::SqliteReceivableRepository,
};

pub async fn execute(db: &Db, receivable_id: Uuid) -> AppResult<Receivable> {
    let mut tx = db.begin_tx().await;

    let result = SqliteReceivableRepository::find_by_id(&mut tx, receivable_id)
        .await?
        .ok_or_else(|| not_found("receivable not found"));

    let _ = tx.rollback().await.map_err(infra);

    result
}
