use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::operation_context::OperationContext,
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::reservation::reservation_repository::SqliteReservationRepository,
    usecase::{
        business_date::validation::ensure_active_business_date_closing,
        reservation::command::mark_no_show::mark_no_show_in_tx,
    },
};

pub async fn execute(
    db: &Db,
    reservation_id: Uuid,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let business_date = ensure_active_business_date_closing(&mut tx).await?;
        let reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        if reservation.check_in > business_date.business_date {
            return Err(conflict(
                "night audit no-show arrival must be due by current business date",
            ));
        }

        if reservation.reservation_status == ReservationStatus::NoShow {
            return mark_no_show_in_tx(&mut tx, reservation_id, &context).await;
        }

        if reservation.reservation_status != ReservationStatus::Confirmed
            || reservation.stay_status != Some(StayStatus::Confirmed)
        {
            return Err(conflict(
                "only unresolved confirmed arrivals can be marked no-show during night audit",
            ));
        }

        mark_no_show_in_tx(&mut tx, reservation_id, &context).await
    }
    .await;

    match result {
        Ok(reservation) => {
            tx.commit().await.map_err(infra)?;

            Ok(reservation)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
