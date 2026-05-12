use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::{
        folio::FolioStatus,

        invoice::Invoice,

        receivable::Receivable,

        settlement_transition::{
            SettlementTransition,
            SettlementTransitionType,
        },
    },

    error::app_error::{
        AppResult,
        conflict,
        infra,
        not_found,
        validation,
    },

    repository::sqlite::{
        behavioral::
            settlement_transition_repository::
                SqliteSettlementTransitionRepository,

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
    },

    usecase::billing::calculation::
        calculate_balance::
            calculate_balance_in_tx,
};

pub async fn issue_invoice(
    db: &Db,
    folio_id: Uuid,
) -> AppResult<(Uuid, Uuid)> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let folio =
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

        if folio.status
            != FolioStatus::Closed
        {

            return Err(
                conflict(
                    "cannot issue invoice for open folio"
                )
            );
        }

        let billing_account_id =
            folio
                .billing_account_id
                .ok_or(
                    conflict(
                        "billing account not assigned"
                    )
                )?;

        SqliteBillingAccountRepository
            ::find_by_id(
                &mut tx,
                billing_account_id,
            )
            .await?
            .ok_or(
                not_found(
                    "billing account not found"
                )
            )?;

        if SqliteInvoiceRepository
            ::find_by_folio_id(
                &mut tx,
                folio.id,
            )
            .await?
            .is_some()
        {

            return Err(
                conflict(
                    "invoice already exists for folio"
                )
            );
        }

        let issued_amount =
            calculate_balance_in_tx(
                &mut tx,
                folio.id,
            )
            .await?;

        let invoice =
            Invoice::issue(
                Uuid::new_v4(),
                folio.id,
                billing_account_id,
                issued_amount,
            )
            .map_err(validation)?;

        SqliteInvoiceRepository
            ::save(
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
            .map_err(validation)?;

        SqliteReceivableRepository
            ::save(
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

        SqliteSettlementTransitionRepository
            ::save(
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

        SqliteSettlementTransitionRepository
            ::save(
                &mut tx,
                &receivable_opened_transition,
            )
            .await?;

        Ok((
            invoice.id,
            receivable.id,
        ))

    }.await;

    match result {

        Ok(ids) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(ids)
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}