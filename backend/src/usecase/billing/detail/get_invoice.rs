use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::invoice::Invoice,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::billing::{
        invoice_repository::SqliteInvoiceRepository,
        receivable_repository::SqliteReceivableRepository,
    },
};

pub struct InvoiceDetail {
    pub invoice: Invoice,
    pub receivable_id: Option<Uuid>,
}

pub async fn execute(db: &Db, invoice_id: Uuid) -> AppResult<InvoiceDetail> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let invoice = SqliteInvoiceRepository::find_by_id(&mut tx, invoice_id)
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        let receivable_id = SqliteReceivableRepository::find_by_invoice_id(&mut tx, invoice.id)
            .await?
            .map(|receivable| receivable.id);

        Ok(InvoiceDetail {
            invoice,
            receivable_id,
        })
    }
    .await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
