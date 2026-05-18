use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_inventory_aggregates::materialize_inventory_aggregates,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_inventory_aggregates_by_date::delete_inventory_aggregates_by_date,
        save_inventory_aggregate::save_inventory_aggregate,
    },
};

pub async fn refresh_inventory_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    push_trace(ProjectionNode::InventoryAggregate);

    delete_inventory_aggregates_by_date(tx, service_date).await?;

    let aggregates = materialize_inventory_aggregates(tx, service_date).await?;

    for aggregate in aggregates {
        save_inventory_aggregate(tx, &aggregate).await?;
    }

    Ok(())
}
