use chrono::Utc;
use uuid::Uuid;

use crate::{
    api::dto::billing::input::create_billing_account_input::CreateBillingAccountInput,
    db::connection::Db,
    domain::entity::billing_account::{
        BillingAccount,
        BillingAccountStatus,
    },
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::billing::billing_account_repository::SqliteBillingAccountRepository,
};

pub async fn execute(
    db: &Db,
    input: CreateBillingAccountInput,
) -> AppResult<BillingAccount> {
    let mut tx = db.begin_tx().await;
    
    let result = async {

        let account = BillingAccount {
            id: Uuid::new_v4(),
            company_id: input.company_id,
            name: input.name,
            status: BillingAccountStatus::Active,
            created_at: Utc::now(),
        };

        SqliteBillingAccountRepository::save(
            &mut tx,
            &account,
        )
        .await?;

        Ok(account)
    }
    .await;

    match result {
        Ok(account) => {
            tx.commit().await.map_err(infra)?;

            Ok(account)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
    
}