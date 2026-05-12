use async_trait::async_trait;

use sqlx::{
    Sqlite,
    Transaction
};

use crate::{
    error::app_error::AppResult,
    
    projection::{
        invalidation::projection_invalidation
            ::ProjectionRefreshTarget,

        topology::projection_node::
            ProjectionNode,
    },
};

#[async_trait]
pub trait ProjectionConvergenceExecutor {

    async fn execute(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        node: ProjectionNode,
        target: &ProjectionRefreshTarget,
    ) -> AppResult<()>;

    async fn execute_rebuild(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        node: ProjectionNode,
    ) -> AppResult<()>;
}