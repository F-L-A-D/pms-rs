use crate::{
    api::dto::billing::input::assign_billing_account_input::AssignBillingAccountInput,
    db::connection::Db,
    domain::entity::folio::{Folio, FolioStatus},
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::billing::{
        billing_account_repository::SqliteBillingAccountRepository,
        folio_repository::SqliteFolioRepository,
    },
};

pub async fn execute(db: &Db, input: AssignBillingAccountInput) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut folio = match SqliteFolioRepository::find_by_id(&mut tx, input.folio_id).await? {
            Some(folio) => folio,

            None => {
                return Err(not_found("folio not found"));
            }
        };

        if !matches!(folio.status, FolioStatus::Open) {
            return Err(conflict(
                "cannot change billing responsibility unless folio is open",
            ));
        }

        SqliteBillingAccountRepository::find_by_id(&mut tx, input.billing_account_id)
            .await?
            .ok_or_else(|| not_found("billing account not found"))?;

        folio.billing_account_id = Some(input.billing_account_id);

        SqliteFolioRepository::save(&mut tx, &folio).await?;

        Ok(folio)
    }
    .await;

    match result {
        Ok(folio) => {
            tx.commit().await.map_err(infra)?;

            Ok(folio)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
