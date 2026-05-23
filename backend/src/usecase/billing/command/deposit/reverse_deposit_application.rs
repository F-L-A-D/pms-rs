use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::reverse_deposit_application_input::ReverseDepositApplicationInput,
    db::connection::Db,
    domain::{
        entity::receivable::ReceivableStatus,
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            settlement_transition::{SettlementTransition, SettlementTransitionType},
        },
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::{
        behavioral::settlement_transition_repository::SqliteSettlementTransitionRepository,
        operational::{
            billing::{
                deposit_application_repository::SqliteDepositApplicationRepository,
                deposit_repository::SqliteDepositRepository,
                invoice_repository::SqliteInvoiceRepository,
                receivable_repository::SqliteReceivableRepository,
            },
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
        },
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: ReverseDepositApplicationInput) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut application =
            SqliteDepositApplicationRepository::find_by_id(&mut tx, input.deposit_application_id)
                .await?
                .ok_or_else(|| not_found("deposit application not found"))?;

        if application.reversed_at.is_some() {
            return Err(conflict("deposit application already reversed"));
        }

        let mut deposit = SqliteDepositRepository::find_by_id(&mut tx, application.deposit_id)
            .await?
            .ok_or_else(|| not_found("deposit not found"))?;

        let mut receivable =
            SqliteReceivableRepository::find_by_id(&mut tx, application.receivable_id)
                .await?
                .ok_or_else(|| not_found("receivable not found"))?;

        let invoice = SqliteInvoiceRepository::find_by_id(&mut tx, receivable.invoice_id)
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        if deposit.folio_id != invoice.folio_id {
            return Err(conflict(
                "deposit and receivable belong to different folios",
            ));
        }

        match receivable.status {
            ReceivableStatus::Open | ReceivableStatus::Settled => {}

            ReceivableStatus::Disputed => {
                return Err(conflict(
                    "cannot reverse deposit application for disputed receivable",
                ));
            }

            ReceivableStatus::WrittenOff => {
                return Err(conflict(
                    "cannot reverse deposit application for written off receivable",
                ));
            }

            ReceivableStatus::Voided => {
                return Err(conflict(
                    "cannot reverse deposit application for voided receivable",
                ));
            }
        }

        let before_reversed_at = application.reversed_at;

        let before_deposit_unapplied_amount = deposit.unapplied_amount;

        let before_deposit_status = deposit.status;

        let before_receivable_status = receivable.status;

        let before_receivable_outstanding_amount = receivable.outstanding_amount;

        let before_json = serde_json::json!({
            "deposit_application_id": application.id,
            "deposit_id": application.deposit_id,
            "receivable_id": application.receivable_id,
            "folio_id": invoice.folio_id,
            "application_amount": application.amount,
            "deposit_application_reversed_at": application.reversed_at,
            "deposit_unapplied_amount": deposit.unapplied_amount,
            "deposit_status": deposit.status,
            "receivable_outstanding_amount": receivable.outstanding_amount,
            "receivable_status": receivable.status,
        })
        .to_string();

        deposit
            .reverse_application(application.amount)
            .map_err(conflict)?;

        application.reverse(Utc::now()).map_err(conflict)?;

        let new_outstanding_amount = receivable.outstanding_amount + application.amount;

        receivable.reopen_with_outstanding_amount(new_outstanding_amount);

        SqliteDepositRepository::save(&mut tx, &deposit).await?;

        SqliteDepositApplicationRepository::save(&mut tx, &application).await?;

        SqliteReceivableRepository::save(&mut tx, &receivable).await?;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::DepositApplicationReversed,
                amount: application.amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        let context = OperationContext::api_system();

        let after_json = serde_json::json!({
            "deposit_application_id": application.id,
            "deposit_id": application.deposit_id,
            "receivable_id": application.receivable_id,
            "folio_id": invoice.folio_id,
            "application_amount": application.amount,
            "deposit_application_reversed_at": application.reversed_at,
            "deposit_unapplied_amount": deposit.unapplied_amount,
            "deposit_status": deposit.status,
            "receivable_outstanding_amount": receivable.outstanding_amount,
            "receivable_status": receivable.status,
        })
        .to_string();

        let mut changed_fields = Vec::new();

        if before_reversed_at != application.reversed_at {
            changed_fields.push(ChangedField::new(
                "deposit_application_reversed_at",
                before_reversed_at.map(|value| value.to_rfc3339()),
                application.reversed_at.map(|value| value.to_rfc3339()),
            ));
        }

        if before_deposit_unapplied_amount != deposit.unapplied_amount {
            changed_fields.push(ChangedField::new(
                "deposit_unapplied_amount",
                Some(before_deposit_unapplied_amount.to_string()),
                Some(deposit.unapplied_amount.to_string()),
            ));
        }

        if before_deposit_status != deposit.status {
            changed_fields.push(ChangedField::new(
                "deposit_status",
                Some(before_deposit_status.to_snake().to_string()),
                Some(deposit.status.to_snake().to_string()),
            ));
        }

        if before_receivable_outstanding_amount != receivable.outstanding_amount {
            changed_fields.push(ChangedField::new(
                "receivable_outstanding_amount",
                Some(before_receivable_outstanding_amount.to_string()),
                Some(receivable.outstanding_amount.to_string()),
            ));
        }

        if before_receivable_status != receivable.status {
            changed_fields.push(ChangedField::new(
                "receivable_status",
                Some(before_receivable_status.to_snake().to_string()),
                Some(receivable.status.to_snake().to_string()),
            ));
        }

        let changed_fields_json = serde_json::to_string(&changed_fields).map_err(infra)?;

        let operation_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "deposit_application".to_string(),
            aggregate_id: application.id,
            operation_type: OperationType::ReverseDepositApplication,
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
                aggregate_type: "deposit_application".to_string(),
                aggregate_id: application.id,
                action: "deposit.application.reverse".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit().await.map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
