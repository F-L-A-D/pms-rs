use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::CreateReservationTraceInput,
    db::connection::Db,
    domain::semantic::{
        operation_context::OperationContext,
        reservation_trace::{ReservationTrace, ReservationTraceKind},
    },
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::{
        reservation_repository::SqliteReservationRepository,
        reservation_trace_repository::SqliteReservationTraceRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(
    db: &Db,
    input: CreateReservationTraceInput,
    context: OperationContext,
) -> AppResult<ReservationTrace> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let reservation = SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let body = normalize_required_string(input.body, "body")?;
        let department_code = normalize_optional_string(input.department_code)
            .ok_or_else(|| validation("department_code required"))?;

        let trace = ReservationTrace {
            id: Uuid::new_v4(),
            reservation_id: reservation.id,
            kind: ReservationTraceKind::DepartmentTrace,
            department_code: Some(department_code),
            body,
            actor_id: input.actor_id.and_then(normalize_optional_string),
            created_at: Utc::now(),
            resolved_at: None,
            resolved_by: None,
            deleted_at: None,
            deleted_by: None,
        };

        SqliteReservationTraceRepository::save(&mut tx, &trace).await?;

        let after_json = serde_json::json!({
            "id": trace.id,
            "reservation_id": trace.reservation_id,
            "kind": trace.kind,
            "department_code": trace.department_code,
            "body": trace.body,
            "actor_id": trace.actor_id,
            "created_at": trace.created_at,
        })
        .to_string();

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "reservation".to_string(),
                aggregate_id: reservation.id,
                action: "reservation.trace.add".to_string(),
                before_json: None,
                after_json,
                changed_fields_json: "[]".to_string(),
                reason: None,
            },
        )
        .await?;

        Ok(trace)
    }
    .await;

    match result {
        Ok(trace) => {
            tx.commit().await.map_err(infra)?;

            Ok(trace)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}

fn normalize_required_string(value: String, field_name: &str) -> AppResult<String> {
    let trimmed = value.trim().to_string();

    if trimmed.is_empty() {
        Err(validation(format!("{field_name} required")))
    } else {
        Ok(trimmed)
    }
}

fn normalize_optional_string(value: String) -> Option<String> {
    let trimmed = value.trim().to_string();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
