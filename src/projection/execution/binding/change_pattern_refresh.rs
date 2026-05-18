use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        signal::refresh::refresh_change_pattern::refresh_change_pattern,
    },
};

pub async fn execute_change_pattern_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    if let ProjectionRefreshTarget::OperationEvent { event_id } = target {
        refresh_change_pattern(tx, *event_id).await?;
    }

    Ok(())
}
