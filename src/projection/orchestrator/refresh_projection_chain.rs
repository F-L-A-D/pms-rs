use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{
        invalidation::{
            projection_invalidation::
                ProjectionInvalidation,

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

            projection_ordering::
                invalidation_traversal_plan,
        },
    },
};

async fn refresh_single_projection(
    tx: &mut Transaction<'_, Sqlite>,
    node: ProjectionNode,
    scope: &ProjectionScope,
) -> AppResult<()>
{
    match node {

        ProjectionNode::HotelInventory => {

            match scope {

                ProjectionScope::Date { date } => {

                    refresh_hotel_inventory_projection(
                        tx,
                        date,
                    )
                    .await?;
                }

                ProjectionScope::Global => {
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

    for node in plan.ordered_nodes {

        refresh_single_projection(
            tx,
            node,
            &plan.scope,
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

            ProjectionScope::Date {
                date: date.to_string(),
            },
        ),
    )
    .await
}