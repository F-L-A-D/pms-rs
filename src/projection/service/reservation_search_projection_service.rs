use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::error::app_error::
    AppResult;

use crate::projection::materializer::
    reservation_search_materializer::
    materialize_reservation_search_projection;

use crate::repository::sqlite::projection::
    reservation_search_projection_repository::
    ReservationSearchProjectionRepository;

pub async fn refresh_reservation_search_projection(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
) -> AppResult<()> {

    let projection =
        materialize_reservation_search_projection(
            tx,
            reservation_id,
        )
        .await?;

    ReservationSearchProjectionRepository
        ::upsert(
            tx,
            &projection,
        )
        .await?;

    Ok(())
}