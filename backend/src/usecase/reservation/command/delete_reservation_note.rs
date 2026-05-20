use chrono::Utc;

use crate::{
    api::dto::input::reservation::DeleteReservationNoteInput,
    db::connection::Db,
    domain::semantic::operation_context::OperationContext,
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        reservation_note_repository::SqliteReservationNoteRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(
    db: &Db,
    input: DeleteReservationNoteInput,
    context: OperationContext,
) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let note = SqliteReservationNoteRepository::find_by_id(&mut tx, input.note_id)
            .await?
            .ok_or_else(|| not_found("reservation note not found"))?;

        if note.reservation_id != input.reservation_id {
            return Err(not_found("reservation note not found"));
        }

        if note.deleted_at.is_some() {
            return Err(conflict("reservation note already deleted"));
        }

        let actor_id = input.actor_id.and_then(normalize_optional_string);
        let deleted_at = Utc::now();

        SqliteReservationNoteRepository::mark_deleted(
            &mut tx,
            note.id,
            deleted_at,
            actor_id.as_deref(),
        )
        .await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "reservation".to_string(),
                aggregate_id: input.reservation_id,
                action: "reservation.memo.delete".to_string(),
                before_json: Some(note_json(&note).to_string()),
                after_json: serde_json::json!({
                    "id": note.id,
                    "deleted_at": deleted_at,
                    "deleted_by": actor_id,
                })
                .to_string(),
                changed_fields_json: "[]".to_string(),
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

fn note_json(
    note: &crate::domain::semantic::reservation_note::ReservationNote,
) -> serde_json::Value {
    serde_json::json!({
        "id": note.id,
        "reservation_id": note.reservation_id,
        "kind": note.kind,
        "body": note.body,
        "actor_id": note.actor_id,
        "created_at": note.created_at,
        "deleted_at": note.deleted_at,
        "deleted_by": note.deleted_by,
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
