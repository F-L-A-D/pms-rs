use uuid::Uuid;

use crate::db::connection::Db;

use crate::domain::folio_entry::{EntryType, FolioEntry};

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::error::app_error::{AppError, AppResult};

use crate::repository::sqlite::operational::{
    folio_entry_repository::SqliteFolioEntryRepository, folio_repository::SqliteFolioRepository,
    reservation_repository::SqliteReservationRepository,
};

use crate::usecase::timeline::record_event::record_event;

use crate::projection::service::guest_summary_projection_service::refresh_guest_summary_projection;

pub async fn post_room_charge(
    db: &Db,
    folio_id: Uuid,
    amount: i64,
    description: Option<String>,
) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let folio = SqliteFolioRepository::find_by_id(&mut tx, folio_id)
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(AppError::NotFound("folio not found".into()))?;

        let entry = FolioEntry::new(
            Uuid::new_v4(),
            folio_id,
            EntryType::RoomCharge,
            amount,
            description,
        );

        SqliteFolioEntryRepository::save(&mut tx, &entry)
            .await
            .map_err(AppError::Infrastructure)?;

        let res = SqliteReservationRepository::find_by_id(&mut tx, folio.reservation_id)
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(AppError::NotFound("reservation not found".into()))?;

        let primary_guest_id = res.primary_participant().map(|p| p.guest_id);

        if let Some(guest_id) = &primary_guest_id {
            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::RoomChargePosted,
                entry.id,
            )
            .await?;
        }

        for participant in &res.participants {
            refresh_guest_summary_projection(&mut tx, participant.guest_id).await?;
        }

        Ok(())
    }
    .await;

    match result {
        Ok(_) => {
            tx.commit()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Ok(())
        }

        Err(e) => {
            tx.rollback()
                .await
                .map_err(|e| AppError::Infrastructure(e.to_string()))?;

            Err(e)
        }
    }
}
