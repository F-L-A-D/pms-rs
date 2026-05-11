use uuid::Uuid;

use chrono::Utc;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppError,
        AppResult,
    },

    domain::{
        folio::FolioStatus,

        invoice::Invoice,

        receivable::Receivable,

        settlement_transition::{
            SettlementTransition,
            SettlementTransitionType,
        },
    },

    repository::sqlite::{

        operational::{

            billing_account_repository::
                SqliteBillingAccountRepository,

            folio_repository::
                SqliteFolioRepository,

            invoice_repository::
                SqliteInvoiceRepository,

            receivable_repository::
                SqliteReceivableRepository,
        },

        behavioral::
            settlement_transition_repository::
                SqliteSettlementTransitionRepository,
    },

    usecase::billing::calculate_balance::
        calculate_balance_in_tx,
};

pub async fn issue_invoice(
    db: &Db,
    folio_id: Uuid,

) -> AppResult<(Uuid, Uuid)> {

    let mut tx =
        db.begin_tx().await;

    let folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            folio_id,
        )
        .await?
        .ok_or(
            AppError::NotFound(
                "folio not found".into()
            )
        )?;

    if folio.status != FolioStatus::Closed {

        return Err(
            AppError::Conflict(
                "cannot issue invoice for open folio"
                    .into()
            )
        );
    }

    let billing_account_id =
        folio.billing_account_id
            .ok_or(
                AppError::Conflict(
                    "billing account not assigned"
                        .into()
                )
            )?;

    SqliteBillingAccountRepository::find_by_id(
        &mut tx,
        billing_account_id,
    )
    .await?
    .ok_or(
        AppError::NotFound(
            "billing account not found"
                .into()
        )
    )?;

    if SqliteInvoiceRepository::find_by_folio_id(
        &mut tx,
        folio.id,
    )
    .await?
    .is_some()
    {
        return Err(
            AppError::Conflict(
                "invoice already exists for folio"
                    .into()
            )
        );
    }

    let issued_amount =
        calculate_balance_in_tx(
            &mut tx,
            folio.id,
        )
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    let invoice =
        Invoice::issue(
            Uuid::new_v4(),
            folio.id,
            billing_account_id,
            issued_amount,
        )
        .map_err(
            AppError::Validation
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
        )
        .map_err(
            AppError::Validation
        )?;

    SqliteReceivableRepository::save(
        &mut tx,
        &receivable,
    )
    .await?;

    let invoice_issued_transition =
        SettlementTransition {

            id:
                Uuid::new_v4(),

            receivable_id:
                receivable.id,

            transition_type:
                SettlementTransitionType::
                    InvoiceIssued,

            amount:
                invoice.issued_amount,

            occurred_at:
                Utc::now(),
        };

    SqliteSettlementTransitionRepository::insert(
        &mut tx,
        &invoice_issued_transition,
    )
    .await?;

    let receivable_opened_transition =
        SettlementTransition {

            id:
                Uuid::new_v4(),

            receivable_id:
                receivable.id,

            transition_type:
                SettlementTransitionType::
                    ReceivableOpened,

            amount:
                invoice.issued_amount,

            occurred_at:
                Utc::now(),
        };

    SqliteSettlementTransitionRepository::insert(
        &mut tx,
        &receivable_opened_transition,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    Ok((
        invoice.id,
        receivable.id,
    ))
}