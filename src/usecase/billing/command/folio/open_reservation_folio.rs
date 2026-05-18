use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::billing::input::open_reservation_folio_input::OpenReservationFolioInput,
    db::connection::Db,
    domain::entity::{
        folio::{Folio, FolioStatus},
        reservation::ReservationStatus,
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::operational::{
        folio_repository::SqliteFolioRepository,
        reservation_repository::SqliteReservationRepository,
    },
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
