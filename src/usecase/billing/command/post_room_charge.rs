use uuid::Uuid;

use crate::{
    db::connection::Db,

    domain::{
        folio_entry::{
            FolioEntry,
            FolioEntryType,
        },

        guest_timeline_event::
            TimelineEventType,
    },

    error::app_error::{
        AppResult,
        infra,
        not_found,
    },

    projection::service::
        guest_summary_projection_service::
            refresh_guest_summary_projection,

    repository::sqlite::operational::{
        folio_entry_repository::
            SqliteFolioEntryRepository,

        folio_repository::
            SqliteFolioRepository,

        reservation_repository::
            SqliteReservationRepository,
    },

    usecase::timeline::command::record_event::record_event,
};

pub async fn post_room_charge(
    db: &Db,
    folio_id: Uuid,
    amount: i64,
    description: Option<String>,
) -> AppResult<()> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let folio =
            SqliteFolioRepository
                ::find_by_id(
                    &mut tx,
                    folio_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "folio not found"
                    )
                )?;

        let entry =
            FolioEntry::new(
                Uuid::new_v4(),

                folio_id,

                FolioEntryType::RoomCharge,

                amount,

                description,
            );

        SqliteFolioEntryRepository
            ::save(
                &mut tx,
                &entry,
            )
            .await?;

        let reservation =
            SqliteReservationRepository
                ::find_by_id(
                    &mut tx,
                    folio.reservation_id,
                )
                .await?
                .ok_or(
                    not_found(
                        "reservation not found"
                    )
                )?;

        let primary_guest_id =
            reservation
                .primary_participant()
                .map(|p| p.guest_id);

        if let Some(guest_id)
            = primary_guest_id
        {

            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::
                    RoomChargePosted,
                entry.id,
            )
            .await?;
        }

        for participant in
            &reservation.participants
        {

            refresh_guest_summary_projection(
                &mut tx,
                participant.guest_id,
            )
            .await?;
        }

        Ok(())

    }.await;

    match result {

        Ok(_) => {

            tx.commit()
                .await
                .map_err(infra)?;

            Ok(())
        }

        Err(e) => {

            let _ =
                tx.rollback().await;

            Err(e)
        }
    }
}