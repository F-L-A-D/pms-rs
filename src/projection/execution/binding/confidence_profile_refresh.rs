use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        signal::refresh::refresh_confidence_profile::refresh_confidence_profile,
    },
};

pub async fn execute_confidence_profile_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    if let ProjectionRefreshTarget::OperationEvent { event_id } = target {
        refresh_confidence_profile(tx, *event_id).await?;
    }

    Ok(())
}
