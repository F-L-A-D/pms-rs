use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::open_reservation_folio_input::OpenReservationFolioInput,
    db::connection::Db,
    domain::entity::{
        folio::{Folio, FolioStatus},
        reservation::ReservationStatus,
    },
    domain::semantic::operation_context::OperationContext,
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        folio_repository::SqliteFolioRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: OpenReservationFolioInput) -> AppResult<Folio> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let reservation = SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        if !matches!(
            reservation.reservation_status,
            ReservationStatus::Confirmed | ReservationStatus::NoShow
        ) {
            return Err(conflict("cannot open folio for inactive reservation"));
        }

        let existing_folios =
            SqliteFolioRepository::list_by_reservation_id(&mut tx, input.reservation_id).await?;

        if existing_folios
            .iter()
            .any(|folio| matches!(folio.status, FolioStatus::Open | FolioStatus::Locked))
        {
            return Err(conflict("reservation already has an active folio"));
        }

        let folio = Folio {
            id: Uuid::new_v4(),
            reservation_id: input.reservation_id,
            billing_account_id: None,
            status: FolioStatus::Open,
            created_at: Utc::now(),
        };

        SqliteFolioRepository::save(&mut tx, &folio).await?;

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "folio".to_string(),
                aggregate_id: folio.id,
                action: "folio.open".to_string(),
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
