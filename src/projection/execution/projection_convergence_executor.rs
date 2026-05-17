use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::dispatcher::{dispatch_projection_rebuild, dispatch_projection_refresh},
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        topology::projection_node::ProjectionNode,
    },
};

pub struct ProjectionConvergenceExecutor;

impl ProjectionConvergenceExecutor {
    pub async fn execute_refresh(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
        target: &ProjectionRefreshTarget,
    ) -> AppResult<()> {
        dispatch_projection_refresh(tx, node, target).await
    }

    pub async fn execute_rebuild(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
    ) -> AppResult<()> {
        dispatch_projection_rebuild(tx, node).await
    }
}
