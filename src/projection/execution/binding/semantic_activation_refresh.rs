use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        signal::refresh::refresh_semantic_activation::refresh_semantic_activation,
    },
};

pub async fn execute_semantic_activation_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    if let ProjectionRefreshTarget::OperationEvent { event_id } = target {
        refresh_semantic_activation(tx, *event_id).await?;
    }

    Ok(())
}
