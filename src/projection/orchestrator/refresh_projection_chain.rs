use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{
        invalidation::{
            projection_invalidation::{
                ProjectionInvalidation,
                ProjectionRefreshTarget,
            },

            projection_scope::
                ProjectionScope,
        },

        service::{
            hotel_inventory_projection_service::
                refresh_hotel_inventory_projection,
        },

        topology::{
            projection_node::
                ProjectionNode,

            invalidation_traversal_planner::
                invalidation_traversal_plan,
        },
    },
};

async fn refresh_single_projection(
    tx: &mut Transaction<'_, Sqlite>,
    node: ProjectionNode,
    target: &ProjectionRefreshTarget,
) -> AppResult<()>
{
    match node {

        ProjectionNode::HotelInventory => {

            match target {

                ProjectionRefreshTarget::InventoryDate {
                    date
                } => {

                    refresh_hotel_inventory_projection(
                        tx,
                        date,
                    )
                    .await?;
                }

                ProjectionRefreshTarget::Global => {
                    // no-op
                }
            }
        }

        _ => {
            // no-op
        }
    }

    Ok(())
}

pub async fn propagate_invalidation(
    tx: &mut Transaction<'_, Sqlite>,
    invalidation: ProjectionInvalidation,
) -> AppResult<()>
{
    let plan =
        invalidation_traversal_plan(
            &invalidation
        );

    for node in
    plan
        .affected_subgraph
        .nodes
    {

        refresh_single_projection(
            tx,
            node,
            &invalidation.target,
        )
        .await?;
    }

    Ok(())
}

pub async fn refresh_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    start: ProjectionNode,
    date: &str,
) -> AppResult<()>
{
    propagate_invalidation(
        tx,

        ProjectionInvalidation::new(
            start,

            ProjectionScope::Inventory,

            ProjectionRefreshTarget::InventoryDate {
                date: date.to_string(),
            },
        ),
    )
    .await
}