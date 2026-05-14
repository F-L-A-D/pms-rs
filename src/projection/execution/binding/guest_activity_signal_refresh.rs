use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        invalidation::
            projection_invalidation::
                ProjectionRefreshTarget,

        signal::refresh::
            refresh_guest_activity_signal::
                refresh_guest_activity_signal,
    },
};

pub async fn execute_guest_activity_signal_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()>
{
    refresh_guest_activity_signal(
        tx,
        target,
    )
    .await
}