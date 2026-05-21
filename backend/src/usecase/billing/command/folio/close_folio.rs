use crate::{
    api::dto::billing::input::close_folio_input::CloseFolioInput,
    db::connection::Db,
    domain::{
        entity::folio::{Folio, FolioStatus},
        semantic::operation_context::OperationContext,
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::billing::folio_repository::SqliteFolioRepository,
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: CloseFolioInput) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut folio = SqliteFolioRepository::find_by_id(&mut tx, input.folio_id)
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        match folio.status {
            FolioStatus::Open | FolioStatus::Locked => {
                folio.status = FolioStatus::Closed;
            }

            FolioStatus::Closed => {
                return Err(conflict("folio already closed"));
            }
        }

        SqliteFolioRepository::save(&mut tx, &folio).await?;

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "folio".to_string(),
                aggregate_id: folio.id,
                action: "folio.close".to_string(),
                before_json: None,
                after_json: serde_json::json!({
                    "id": folio.id,
                    "reservation_id": folio.reservation_id,
                    "status": folio.status,
                    "billing_account_id": folio.billing_account_id,
                })
                .to_string(),
                changed_fields_json: serde_json::json!([
                    {"field_name": "status", "before_value": null, "after_value": folio.status.to_snake()}
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
