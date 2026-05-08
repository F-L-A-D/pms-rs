use::chrono::Utc;

use uuid::Uuid;

use sqlx::{
    Transaction,
    Sqlite,
};

use crate::projection::crm::guest_summary::GuestSummaryProjection;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::operational::{
    folio_entry_repository::SqliteFolioEntryRepository,
    folio_repository::SqliteFolioRepository,
    reservation_repository::SqliteReservationRepository,
};

pub async fn get_guest_summary(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestSummaryProjection> {

    let reservations =
        SqliteReservationRepository::find_by_guest_id(
            tx,
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
                tx,
                &reservation.id,
            )
            .await
            .map_err(AppError::Infrastructure)?;

        for folio in folios {

            let entries =
                SqliteFolioEntryRepository::find_by_folio_id(
                    tx,
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

    Ok(GuestSummaryProjection {
        guest_id,
        total_stays,
        total_nights,
        total_spending,
        last_stay_at,
        projection_version: 1,
        updated_at: Utc::now(),
    })
}