use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        aggregate::refresh::
            refresh_guest_aggregate::
                refresh_guest_aggregate,

        invalidation::projection_invalidation::
            ProjectionRefreshTarget,
    },
};

pub async fn execute_guest_aggregate_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()>
{
    match target {

        ProjectionRefreshTarget::Guest {
            guest_id,
        } => {

            refresh_guest_aggregate(
                tx,
                *guest_id,
            )
            .await?;
        }

        ProjectionRefreshTarget::Global => {}

        _ => {}
    }

    Ok(())
}