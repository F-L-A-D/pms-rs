use crate::db::connection::Db;

use crate::domain::guest_metrics::GuestMetrics;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::{
    folio_entry_repository::SqliteFolioEntryRepository,
    folio_repository::SqliteFolioRepository,
    reservation_repository::SqliteReservationRepository,
};

pub async fn get_guest_metrics(
    db: &Db,
    guest_id: &str,
) -> AppResult<GuestMetrics> {

    let mut tx =
        db.begin_tx().await;

    let result = async {

        let reservations =
            SqliteReservationRepository::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        let total_stays =
            reservations.len() as i64;

        let total_nights =
            reservations
                .iter()
                .map(|r| r.nights().len() as i64)
                .sum();

        let last_stay_at =
            reservations
                .iter()
                .map(|r| r.check_in)
                .max();

        let mut total_spending = 0;

        for reservation in &reservations {

            let folios =
                SqliteFolioRepository::find_by_reservation_id(
                    &mut tx,
                    &reservation.id,
                )
                .await
                .map_err(AppError::Infrastructure)?;

            for folio in folios {

                let entries =
                    SqliteFolioEntryRepository::find_by_folio_id(
                        &mut tx,
                        &folio.id,
                    )
                    .await
                    .map_err(AppError::Infrastructure)?;

                total_spending +=
                    entries
                        .iter()
                        .filter(|e| e.amount > 0)
                        .map(|e| e.amount)
                        .sum::<i64>();
            }
        }

        Ok(
            GuestMetrics {
                total_stays,
                total_nights,
                total_spending,
                last_stay_at,
            }
        )

    }.await;

    tx.rollback()
        .await
        .map_err(|e| {
            AppError::Infrastructure(
                e.to_string()
            )
        })?;

    result
}