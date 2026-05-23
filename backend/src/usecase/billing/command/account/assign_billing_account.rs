use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::assign_billing_account_input::AssignBillingAccountInput,
    db::connection::Db,
    domain::{
        entity::folio::{Folio, FolioStatus},
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
        },
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        billing::{
            billing_account_repository::SqliteBillingAccountRepository,
            folio_repository::SqliteFolioRepository,
        },
        operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
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

        let context = OperationContext::api_system();

        let before_billing_account_id = folio.billing_account_id;

        let before_json = serde_json::json!({
            "id": folio.id,
            "folio_id": folio.id,
            "reservation_id": folio.reservation_id,
            "billing_account_id": before_billing_account_id,
            "status": folio.status,
        })
        .to_string();

        folio.billing_account_id = Some(input.billing_account_id);

        SqliteFolioRepository::save(&mut tx, &folio).await?;

        let after_json = serde_json::json!({
            "id": folio.id,
            "folio_id": folio.id,
            "reservation_id": folio.reservation_id,
            "billing_account_id": folio.billing_account_id,
            "status": folio.status,
        })
        .to_string();

        let changed_fields_json = serde_json::to_string(&vec![ChangedField::new(
            "billing_account_id",
            before_billing_account_id.map(|id| id.to_string()),
            folio.billing_account_id.map(|id| id.to_string()),
        )])
        .map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "folio".to_string(),
            aggregate_id: folio.id,
            operation_type: OperationType::AssignBillingAccount,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: Some(before_json.clone()),
            after_json: after_json.clone(),
            changed_fields_json: changed_fields_json.clone(),
            occurred_at: Utc::now(),
        };

        SqliteOperationChangeEventRepository::save(&mut tx, &operation_event).await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "folio".to_string(),
                aggregate_id: folio.id,
                action: "folio.assign_billing_account".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
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
