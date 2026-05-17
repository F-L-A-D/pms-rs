use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::binding::{
            guest_activity_signal_rebuild::execute_guest_activity_signal_rebuild,
            guest_activity_signal_refresh::execute_guest_activity_signal_refresh,
            guest_aggregate_rebuild::execute_guest_aggregate_rebuild,
            guest_aggregate_refresh::execute_guest_aggregate_refresh,
            housekeeping_daily_workload_aggregate_rebuild::execute_housekeeping_daily_workload_aggregate_rebuild,
            housekeeping_daily_workload_aggregate_refresh::execute_housekeeping_daily_workload_aggregate_refresh,
            inventory_aggregate_rebuild::execute_inventory_aggregate_rebuild,
            inventory_aggregate_refresh::execute_inventory_aggregate_refresh,
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
            ProjectionNode::GuestAggregate => execute_guest_aggregate_refresh(tx, target).await,
            ProjectionNode::GuestActivitySignal => {
                execute_guest_activity_signal_refresh(tx, target).await
            }
            ProjectionNode::HousekeepingDailyWorkloadAggregate => {
                execute_housekeeping_daily_workload_aggregate_refresh(tx, target).await
            }
            ProjectionNode::InventoryAggregate => {
                execute_inventory_aggregate_refresh(tx, target).await
            }
        }
    }

    pub async fn dispatch_rebuild(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
    ) -> AppResult<()> {
        match node {
            ProjectionNode::GuestAggregate => {
                execute_guest_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::GuestActivitySignal => {
                execute_guest_activity_signal_rebuild(tx).await?;
            }
            ProjectionNode::HousekeepingDailyWorkloadAggregate => {
                execute_housekeeping_daily_workload_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::InventoryAggregate => {
                execute_inventory_aggregate_rebuild(tx).await?;
            }
        }

        Ok(())
    }
}
