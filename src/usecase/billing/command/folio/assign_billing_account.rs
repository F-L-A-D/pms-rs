use crate::{
    api::dto::billing::input::assign_billing_account_input::AssignBillingAccountInput,
    db::connection::Db,
    domain::{
        entity::folio::{Folio, FolioStatus},
        semantic::operation_context::OperationContext,
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        billing_account_repository::SqliteBillingAccountRepository,
        folio_repository::SqliteFolioRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
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

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "folio".to_string(),
                aggregate_id: folio.id,
                action: "folio.assign_billing_account".to_string(),
                before_json: None,
                after_json: serde_json::json!({
                    "id": folio.id,
                    "reservation_id": folio.reservation_id,
                    "billing_account_id": folio.billing_account_id,
                    "status": folio.status,
                })
                .to_string(),
                changed_fields_json: serde_json::json!([
                    {"field_name": "billing_account_id", "before_value": null, "after_value": input.billing_account_id.to_string()}
                ])
                .to_string(),
                reason: None,
            },
        )
        .await?;

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
