use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::{
            folio::FolioStatus,
            payment::Payment,
        },
        semantic::{
            operation_change_event::{
                ChangedField,
                OperationChangeEvent,
                OperationType,
            },
            operation_context::OperationContext,
        },
    },
    error::app_error::{
        conflict,
        infra,
        not_found,
        validation,
        AppResult,
    },
    repository::sqlite::{
        operational::{
            billing::{
                payment_repository::SqlitePaymentRepository,
                folio_repository::SqliteFolioRepository,
            },
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
        },
    },
    usecase::audit::command::record_audit_log::{
        record_audit_log,
        RecordAuditLogInput,
    },
};

use crate::api::dto::billing::input::create_payment_input::CreatePaymentInput;

pub async fn execute(
    db: &Db,
    input: CreatePaymentInput,
) -> AppResult<Payment> {
    let mut tx =
        db.begin_tx().await;

    let result = async {
        let folio =
            SqliteFolioRepository::find_by_id(
                &mut tx,
                input.folio_id,
            )
            .await?
            .ok_or_else(|| not_found("folio not found"))?;

        if folio.status != FolioStatus::Open {
            return Err(conflict("payment can only be received for open folio"));
        }

        let payment =
            Payment::new(
                Uuid::new_v4(),
                folio.id,
                input.amount,
                input.method,
                input.external_reference,
                Utc::now(),
            )
            .map_err(conflict)?;

        SqlitePaymentRepository::save(
            &mut tx,
            &payment,
        )
        .await?;

        let context =
            OperationContext::api_system();

        let after_json =
            serde_json::json!({
                "id": payment.id,
                "payment_id": payment.id,
                "folio_id": payment.folio_id,
                "amount": payment.amount,
                "unapplied_amount": payment.unapplied_amount,
                "refunded_amount": payment.refunded_amount,
                "status": payment.status,
                "method": payment.method,
                "external_reference": payment.external_reference,
                "paid_at": payment.paid_at,
            })
            .to_string();

        let changed_fields_json =
            serde_json::to_string(&vec![
                ChangedField::new(
                    "status",
                    None,
                    Some(
                        payment
                            .status
                            .to_snake()
                            .to_string(),
                    ),
                ),
                ChangedField::new(
                    "amount",
                    None,
                    Some(payment.amount.to_string()),
                ),
                ChangedField::new(
                    "unapplied_amount",
                    None,
                    Some(
                        payment
                            .unapplied_amount
                            .to_string(),
                    ),
                ),
            ])
            .map_err(infra)?;

        let operation_event =
            OperationChangeEvent {
                id: Uuid::new_v4(),
                operation_id: context.operation_id,
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                operation_type: OperationType::ReceivePayment,
                actor: context.actor,
                actor_id: context.actor_id.clone(),
                source: context.source,
                before_json: None,
                after_json: after_json.clone(),
                changed_fields_json:
                    changed_fields_json.clone(),
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
                aggregate_type: "payment".to_string(),
                aggregate_id: payment.id,
                action: "payment.receive".to_string(),
                before_json: None,
                after_json,
                changed_fields_json,
                reason: input.reason,
            },
        )
        .await?;

        Ok(payment)
    }
    .await;

    match result {
        Ok(payment) => {
            tx.commit()
                .await
                .map_err(infra)?;

            Ok(payment)
        }

        Err(e) => {
            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}