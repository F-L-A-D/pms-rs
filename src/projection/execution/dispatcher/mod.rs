use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        execution::registry::
            projection_execution_registry::
                ProjectionExecutionRegistry,

        invalidation::projection_invalidation::
            ProjectionRefreshTarget,

        topology::projection_node::ProjectionNode,
    },
};

pub async fn dispatch_projection_refresh(
    tx: &mut Transaction<'_, Sqlite>,
    node: &ProjectionNode,
    target: &ProjectionRefreshTarget,
) -> AppResult<()> {
    ProjectionExecutionRegistry::dispatch_refresh(
        tx,
        node,
        target,
    )
    .await
}

pub async fn dispatch_projection_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
    node: &ProjectionNode,
) -> AppResult<()> {
    ProjectionExecutionRegistry::dispatch_rebuild(
        tx,
        node,
    )
    .await
}