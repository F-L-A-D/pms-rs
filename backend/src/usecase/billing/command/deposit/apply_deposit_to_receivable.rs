use chrono::Utc;

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::apply_deposit_to_receivable_input::ApplyDepositToReceivableInput,
    db::connection::Db,
    domain::{
        entity::{
            deposit_application::DepositApplication,
            receivable::ReceivableStatus,
        },
        semantic::{
            operation_change_event::{
                ChangedField, OperationChangeEvent, OperationType,
            },
            operation_context::OperationContext,
            settlement_transition::{
                SettlementTransition, SettlementTransitionType,
            },
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
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
    usecase::audit::command::record_audit_log::{
        record_audit_log, RecordAuditLogInput,
    },
};

pub struct DepositApplicationResult {
    pub application: DepositApplication,
    pub remaining_outstanding_amount: Decimal,
}

pub async fn execute(
    db: &Db,
    input: ApplyDepositToReceivableInput,
) -> AppResult<DepositApplicationResult> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.amount <= Decimal::ZERO {
            return Err(validation(
                "deposit application amount must be positive",
            ));
        }

        let mut deposit =
            SqliteDepositRepository::find_by_id(
                &mut tx,
                input.deposit_id,
            )
            .await?
            .ok_or_else(|| not_found("deposit not found"))?;

        let mut receivable =
            SqliteReceivableRepository::find_by_id(
                &mut tx,
                input.receivable_id,
            )
            .await?
            .ok_or_else(|| not_found("receivable not found"))?;

        let invoice =
            SqliteInvoiceRepository::find_by_id(
                &mut tx,
                receivable.invoice_id,
            )
            .await?
            .ok_or_else(|| not_found("invoice not found"))?;

        if receivable.status != ReceivableStatus::Open {
            return Err(conflict("receivable is not open"));
        }

        if deposit.folio_id != invoice.folio_id {
            return Err(conflict(
                "deposit and receivable belong to different folios",
            ));
        }

        if input.amount > deposit.unapplied_amount {
            return Err(conflict(
                "deposit application amount exceeds deposit unapplied amount",
            ));
        }

        if input.amount > receivable.outstanding_amount {
            return Err(conflict(
                "deposit application amount exceeds outstanding receivable amount",
            ));
        }

        let before_deposit_unapplied_amount =
            deposit.unapplied_amount;

        let before_deposit_status =
            deposit.status;

        let before_receivable_status =
            receivable.status.clone();

        let before_outstanding_amount =
            receivable.outstanding_amount;

        let before_json =
            serde_json::json!({
                "deposit_id": deposit.id,
                "receivable_id": receivable.id,
                "folio_id": invoice.folio_id,
                "application_id": null,
                "application_amount": input.amount,
                "deposit_unapplied_amount": deposit.unapplied_amount,
                "deposit_status": deposit.status,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
            })
            .to_string();

        deposit
            .apply(input.amount)
            .map_err(conflict)?;

        let application =
            DepositApplication::new(
                Uuid::new_v4(),
                deposit.id,
                receivable.id,
                input.amount,
                Utc::now(),
            )
            .map_err(conflict)?;

        SqliteDepositApplicationRepository::save(
            &mut tx,
            &application,
        )
        .await?;

        receivable.outstanding_amount -= input.amount;

        SqliteSettlementTransitionRepository::save(
            &mut tx,
            &SettlementTransition {
                id: Uuid::new_v4(),
                receivable_id: receivable.id,
                transition_type: SettlementTransitionType::DepositApplied,
                amount: input.amount,
                occurred_at: Utc::now(),
            },
        )
        .await?;

        if receivable.outstanding_amount == Decimal::ZERO {
            receivable
                .settle()
                .map_err(validation)?;

            SqliteSettlementTransitionRepository::save(
                &mut tx,
                &SettlementTransition {
                    id: Uuid::new_v4(),
                    receivable_id: receivable.id,
                    transition_type: SettlementTransitionType::ReceivableSettled,
                    amount: input.amount,
                    occurred_at: Utc::now(),
                },
            )
            .await?;
        }

        SqliteDepositRepository::save(
            &mut tx,
            &deposit,
        )
        .await?;

        SqliteReceivableRepository::save(
            &mut tx,
            &receivable,
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "deposit_id": deposit.id,
                "receivable_id": receivable.id,
                "folio_id": invoice.folio_id,
                "application_id": application.id,
                "application_amount": application.amount,
                "deposit_unapplied_amount": deposit.unapplied_amount,
                "deposit_status": deposit.status,
                "receivable_status": receivable.status,
                "receivable_outstanding_amount": receivable.outstanding_amount,
            })
            .to_string();

        let changed_fields_json =
            serde_json::to_string(&vec![
                ChangedField::new(
                    "deposit_unapplied_amount",
                    Some(before_deposit_unapplied_amount.to_string()),
                    Some(deposit.unapplied_amount.to_string()),
                ),
                ChangedField::new(
                    "deposit_status",
                    Some(before_deposit_status.to_snake().to_string()),
                    Some(deposit.status.to_snake().to_string()),
                ),
                ChangedField::new(
                    "receivable_outstanding_amount",
                    Some(before_outstanding_amount.to_string()),
                    Some(receivable.outstanding_amount.to_string()),
                ),
                ChangedField::new(
                    "receivable_status",
                    Some(before_receivable_status.to_snake().to_string()),
                    Some(receivable.status.to_snake().to_string()),
                ),
            ])
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type: "deposit_application".to_string(),
                aggregate_id: application.id,
                operation_type: OperationType::ApplyDepositToReceivable,
                actor: context.actor,
                actor_id: context.actor_id.clone(),
                source: context.source,
                before_json: Some(before_json.clone()),
                after_json: after_json.clone(),
                changed_fields_json: changed_fields_json.clone(),
                occurred_at: Utc::now(),
            };

        SqliteOperationChangeEventRepository::save(
            &mut tx,
            &operation_event,
        )
        .await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "deposit_application".to_string(),
                aggregate_id: application.id,
                action: "deposit.apply".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(DepositApplicationResult {
            application,
            remaining_outstanding_amount: receivable.outstanding_amount,
        })
    }
    .await;

    match result {
        Ok(result) => {
            tx.commit().await.map_err(infra)?;

            Ok(result)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}