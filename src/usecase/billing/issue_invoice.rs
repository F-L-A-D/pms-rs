use uuid::Uuid;

use serde::Deserialize;

use crate::{
    db::connection::Db,
    domain::{
        folio::FolioStatus,
        invoice::Invoice,
        receivable::Receivable,
    },
    repository::sqlite::operational::{
        billing_account_repository::
            SqliteBillingAccountRepository,

        folio_repository::
            SqliteFolioRepository,

        invoice_repository::
            SqliteInvoiceRepository,

        receivable_repository::
            SqliteReceivableRepository,
    },
    usecase::billing::calculate_balance::
        calculate_balance_in_tx,
};

#[derive(Deserialize)]
pub struct IssueInvoiceInput {
    pub folio_id: Uuid,
}

pub struct IssueInvoiceOutput {
    pub invoice_id: Uuid,
    pub receivable_id: Uuid,
}

pub async fn issue_invoice(
    db: &Db,
    input: IssueInvoiceInput,

) -> Result<IssueInvoiceOutput, String> {

    let mut tx =
        db.begin_tx().await;

    let folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            input.folio_id,
        )
        .await?
        .ok_or(
            "folio not found"
        )?;

    if folio.status != FolioStatus::Closed {

        return Err(
            "cannot issue invoice for open folio"
                .into()
        );
    }

    let billing_account_id =
        folio.billing_account_id
            .ok_or(
                "billing account not assigned"
            )?;

    SqliteBillingAccountRepository::find_by_id(
        &mut tx,
        billing_account_id,
    )
    .await?
    .ok_or(
        "billing account not found"
    )?;

    if SqliteInvoiceRepository::find_by_folio_id(
        &mut tx,
        folio.id,
    )
    .await?
    .is_some()
    {
        return Err(
            "invoice already exists for folio"
                .into()
        );
    }

    let issued_amount =
        calculate_balance_in_tx(
            &mut tx,
            folio.id,
        )
        .await
        .map_err(|e| e.to_string())?;

    let invoice =
        Invoice::issue(
            Uuid::new_v4(),
            folio.id,
            billing_account_id,
            issued_amount,
        )?;

    SqliteInvoiceRepository::save(
        &mut tx,
        &invoice,
    )
    .await?;

    let receivable =
        Receivable::new(
            Uuid::new_v4(),
            invoice.id,
            invoice.issued_amount,
        )?;

    SqliteReceivableRepository::save(
        &mut tx,
        &receivable,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    Ok(IssueInvoiceOutput {

        invoice_id:
            invoice.id,

        receivable_id:
            receivable.id,
    })
}