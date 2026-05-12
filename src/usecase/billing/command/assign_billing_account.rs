use uuid::Uuid;

use crate::{
    db::connection::Db,
    
    error::app_error::{
        AppResult,
        conflict,
        infra,
        not_found,
    }, 
    
    repository::sqlite::operational::{
        billing_account_repository::
            SqliteBillingAccountRepository,

        folio_repository::
            SqliteFolioRepository,
    },
};

pub async fn assign_billing_account(
    db: &Db,
    folio_id: Uuid,
    billing_account_id: Uuid,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let mut folio =
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

        let billing_account =
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

        if folio.billing_account_id
            .is_some()
        {

            return Err(
                conflict(
                    "billing account already assigned"
                )
            );
        }

        folio.billing_account_id =
            Some(
                billing_account.id
            );

        SqliteFolioRepository
            ::save(
                &mut tx,
                &folio,
            )
            .await?;

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}