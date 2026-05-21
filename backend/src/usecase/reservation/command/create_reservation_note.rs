use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::CreateReservationNoteInput,
    db::connection::Db,
    domain::semantic::{
        operation_context::OperationContext,
        reservation_note::{ReservationNote, ReservationNoteKind},
    },
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::reservation::{
        reservation_note_repository::SqliteReservationNoteRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(
    db: &Db,
    input: CreateReservationNoteInput,
    context: OperationContext,
) -> AppResult<ReservationNote> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let reservation = SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let body = normalize_required_string(input.body, "body")?;

        let note = ReservationNote {
            id: Uuid::new_v4(),
            reservation_id: reservation.id,
            kind: ReservationNoteKind::GlobalMemo,
            body,
            actor_id: input.actor_id.and_then(normalize_optional_string),
            created_at: Utc::now(),
            deleted_at: None,
            deleted_by: None,
        };

        SqliteReservationNoteRepository::save(&mut tx, &note).await?;

        let after_json = serde_json::json!({
            "id": note.id,
            "reservation_id": note.reservation_id,
            "kind": note.kind,
            "body": note.body,
            "actor_id": note.actor_id,
            "created_at": note.created_at,
        })
        .to_string();

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "reservation".to_string(),
                aggregate_id: reservation.id,
                action: "reservation.memo.add".to_string(),
                before_json: None,
                after_json,
                changed_fields_json: "[]".to_string(),
                reason: None,
            },
        )
        .await?;

        Ok(note)
    }
    .await;

    match result {
        Ok(note) => {
            tx.commit().await.map_err(infra)?;

            Ok(note)
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
