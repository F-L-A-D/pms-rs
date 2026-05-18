use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::rebuild::rebuild_housekeeping_daily_workload_aggregate::rebuild_housekeeping_daily_workload_aggregate,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_housekeeping_daily_workload_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::HousekeepingDailyWorkloadAggregate).await?;

    rebuild_housekeeping_daily_workload_aggregate(tx).await?;

    Ok(())
}
