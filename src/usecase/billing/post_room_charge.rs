use crate::db::connection::Db;

use crate::domain::folio_entry::{
    EntryType,
    FolioEntry,
};

use crate::domain::guest_timeline_event::TimelineEventType;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::{
    folio_entry_repository::SqliteFolioEntryRepository,
    folio_repository::SqliteFolioRepository,
    reservation_repository::SqliteReservationRepository,
};

use crate::usecase::timeline::record_event::record_event;

pub async fn post_room_charge(
    db: &Db,
    entry_id: String,
    folio_id: &str,
    amount: i64,
    description: Option<String>,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let folio =
            SqliteFolioRepository::find_by_id(
                &mut tx,
                folio_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "folio not found".into()
                )
            )?;

        let entry =
            FolioEntry::new(
                entry_id,
                folio_id.into(),
                EntryType::RoomCharge,
                amount,
                description,
            );

        SqliteFolioEntryRepository::save(
            &mut tx,
            &entry,
        )
        .await
        .map_err(AppError::Infrastructure)?;

        let res =
            SqliteReservationRepository::find_by_id(
                &mut tx,
                &folio.reservation_id,
            )
            .await
            .map_err(AppError::Infrastructure)?
            .ok_or(
                AppError::NotFound(
                    "reservation not found".into()
                )
            )?;

        if let Some(guest_id) =
            &res.primary_guest_id {

            record_event(
                &mut tx,
                guest_id.clone(),
                TimelineEventType::RoomChargePosted,
                entry.id.clone(),
            )
            .await?;
        }

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Ok(())
        }

        Err(e) => {

            tx.rollback()
                .await
                .map_err(|e| {
                    AppError::Infrastructure(
                        e.to_string()
                    )
                })?;

            Err(e)
        }
    }
}