use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{ReservationStatus, StayStatus},
        semantic::guest_timeline_event::TimelineEventType,
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::reservation_repository::SqliteReservationRepository,
    usecase::timeline::command::record_event::record_event,
};

pub async fn check_out(db: &Db, reservation_id: Uuid) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        if reservation.reservation_status != ReservationStatus::Confirmed {
            return Err(conflict("reservation inactive"));
        }

        if reservation.stay_status != Some(StayStatus::CheckedIn) {
            return Err(conflict("invalid stay status"));
        }

        reservation.stay_status = Some(StayStatus::CheckedOut);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        if let Some(guest_id) = reservation.primary_participant().map(|p| p.guest_id) {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::CheckedOut,
                reservation.id,
            )
            .await?;
        }

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
