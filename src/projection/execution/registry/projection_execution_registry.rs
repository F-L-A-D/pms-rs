use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        execution::binding::{
            guest_aggregate_rebuild::execute_guest_aggregate_rebuild,
            guest_aggregate_refresh::execute_guest_aggregate_refresh,
        },
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        topology::projection_node::ProjectionNode,
    },
};

pub struct ProjectionExecutionRegistry;

impl ProjectionExecutionRegistry {
    pub async fn dispatch_refresh(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
        target: &ProjectionRefreshTarget,
    ) -> AppResult<()> {
        match node {
            ProjectionNode::GuestAggregate => {
                execute_guest_aggregate_refresh(
                    tx,
                    target,
                )
                .await
            }
        }
    }

    pub async fn dispatch_rebuild(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
    ) -> AppResult<()> {

        match node {

            ProjectionNode::GuestAggregate => {

                execute_guest_aggregate_rebuild(
                    tx,
                )
                .await?;
            }
        }

        Ok(())
    }
}