use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::invoice::Invoice,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::billing::invoice_repository::SqliteInvoiceRepository,
};

pub async fn execute(db: &Db, invoice_id: Uuid) -> AppResult<Invoice> {
    let mut tx = db.begin_tx().await;

    let result = SqliteInvoiceRepository::find_by_id(&mut tx, invoice_id)
        .await?
        .ok_or_else(|| not_found("invoice not found"));

    let _ = tx.rollback().await.map_err(infra);

    result
}
