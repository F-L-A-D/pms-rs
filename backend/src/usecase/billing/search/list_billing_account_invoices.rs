use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::invoice::Invoice,
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::invoice_repository::SqliteInvoiceRepository,
};

pub async fn execute(db: &Db, billing_account_id: Uuid) -> AppResult<Vec<Invoice>> {
    let mut tx = db.begin_tx().await;

    let result =
        SqliteInvoiceRepository::list_by_billing_account_id(&mut tx, billing_account_id).await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
