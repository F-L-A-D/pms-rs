use chrono::Utc;

use crate::{
    api::dto::input::reservation::ResolveReservationTraceInput,
    db::connection::Db,
    domain::semantic::{operation_context::OperationContext, reservation_trace::ReservationTrace},
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        reservation_repository::SqliteReservationRepository,
        reservation_trace_repository::SqliteReservationTraceRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(
    db: &Db,
    input: ResolveReservationTraceInput,
    context: OperationContext,
) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let trace = SqliteReservationTraceRepository::find_by_id(&mut tx, input.trace_id)
            .await?
            .ok_or_else(|| not_found("reservation trace not found"))?;

        if trace.reservation_id != input.reservation_id {
            return Err(not_found("reservation trace not found"));
        }

        if trace.deleted_at.is_some() {
            return Err(conflict("reservation trace already deleted"));
        }

        if trace.resolved_at.is_some() {
            return Ok(());
        }

        let actor_id = input.actor_id.and_then(normalize_optional_string);
        let resolved_at = Utc::now();

        SqliteReservationTraceRepository::mark_resolved(
            &mut tx,
            trace.id,
            resolved_at,
            actor_id.as_deref(),
        )
        .await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "reservation".to_string(),
                aggregate_id: input.reservation_id,
                action: "reservation.trace.resolve".to_string(),
                before_json: Some(trace_json(&trace).to_string()),
                after_json: serde_json::json!({
                    "id": trace.id,
                    "resolved_at": resolved_at,
                    "resolved_by": actor_id,
                })
                .to_string(),
                changed_fields_json: serde_json::json!(["resolved_at", "resolved_by"]).to_string(),
                reason: None,
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

fn trace_json(trace: &ReservationTrace) -> serde_json::Value {
    serde_json::json!({
        "id": trace.id,
        "reservation_id": trace.reservation_id,
        "kind": trace.kind,
        "department_code": trace.department_code,
        "body": trace.body,
        "actor_id": trace.actor_id,
        "created_at": trace.created_at,
        "resolved_at": trace.resolved_at,
        "resolved_by": trace.resolved_by,
        "deleted_at": trace.deleted_at,
        "deleted_by": trace.deleted_by,
    })
}

fn normalize_optional_string(value: String) -> Option<String> {
    let trimmed = value.trim().to_string();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
