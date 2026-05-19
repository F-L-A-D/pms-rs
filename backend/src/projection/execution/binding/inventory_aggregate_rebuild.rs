use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::rebuild::rebuild_inventory_aggregate::rebuild_inventory_aggregate,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_inventory_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::InventoryAggregate).await?;

    rebuild_inventory_aggregate(tx).await?;

    Ok(())
}
