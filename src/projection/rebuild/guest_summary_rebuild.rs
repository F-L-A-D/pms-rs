use sqlx::{
    Sqlite, 
    Transaction,
};

use crate::error::app_error::AppResult;

use crate::projection::materializer::
    guest_summary_materializer::
        materialize_guest_summary;

use crate::repository::sqlite::operational::
    guest_repository::SqliteGuestRepository;

use crate::repository::sqlite::projection::
    guest_summary_projection_repository::
        GuestSummaryProjectionRepository;

pub async fn rebuild_guest_summary_projection(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    GuestSummaryProjectionRepository
        ::delete_all(tx)
        .await?;

    let guests =
        SqliteGuestRepository
            ::find_all(tx)
            .await?;

    for guest in guests {

        let projection =
            materialize_guest_summary(
                tx,
                guest.id,
            )
            .await?;

        GuestSummaryProjectionRepository
            ::upsert(
                tx,
                &projection,
            )
            .await?;
    }

    Ok(())
}