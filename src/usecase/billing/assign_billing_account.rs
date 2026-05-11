use uuid::Uuid;

use crate::{
    db::connection::Db,

    error::app_error::{
        AppError,
        AppResult,
    },

    repository::sqlite::operational::{

        billing_account_repository::
            SqliteBillingAccountRepository,

        folio_repository::
            SqliteFolioRepository,
    },
};

pub struct AssignBillingAccountInput {

    pub folio_id: Uuid,

    pub billing_account_id: Uuid,
}

pub async fn assign_billing_account(
    db: &Db,
    input: AssignBillingAccountInput,

) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let mut folio =
        SqliteFolioRepository::find_by_id(
            &mut tx,
            input.folio_id,
        )
        .await?
        .ok_or(
            AppError::NotFound(
                "folio not found".into()
            )
        )?;

    let billing_account =
        SqliteBillingAccountRepository::find_by_id(
            &mut tx,
            input.billing_account_id,
        )
        .await?
        .ok_or(
            AppError::NotFound(
                "billing account not found"
                    .into()
            )
        )?;

    if folio.billing_account_id.is_some() {

        return Err(
            AppError::Conflict(
                "billing account already assigned"
                    .into()
            )
        );
    }

    folio.billing_account_id =
        Some(billing_account.id);

    SqliteFolioRepository::save(
        &mut tx,
        &folio,
    )
    .await?;

    tx.commit()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    Ok(())
}