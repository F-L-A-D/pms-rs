use uuid::Uuid;

use sqlx::{
    Sqlite, 
    Transaction,
};


use crate::error::app_error::AppResult;

use crate::projection::materializer::
    guest_summary_materializer::
    materialize_guest_summary;

use crate::repository::sqlite::projection::
    guest_summary_projection_repository::
    GuestSummaryProjectionRepository;

pub async fn refresh_guest_summary_projection(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<()> {

    let projection =
        materialize_guest_summary(
            tx,
            guest_id,
        )
        .await?;

    GuestSummaryProjectionRepository
        ::upsert(
            tx,
            &projection,
        )
        .await?;

    Ok(())
}