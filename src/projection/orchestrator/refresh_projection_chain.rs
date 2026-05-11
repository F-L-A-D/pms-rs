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
                derive_convergence_plan,
        },

        orchestrator::
            convergence_execution_result::
                ConvergenceExecutionResult,
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
) ->AppResult<ConvergenceExecutionResult>
{
    let plan =
        derive_convergence_plan(
            &invalidation
        );

    let mut completed =
    Vec::new();

    for node in
        plan.convergence_nodes()
    {
        let result =
            refresh_single_projection(
                tx,
                *node,
                &invalidation.target,
            )
            .await;

        match result {

            Ok(_) => {

                completed.push(*node);
            }

            Err(_error) => {

                return Ok(
                    ConvergenceExecutionResult::failed(
                        plan.clone(),
                        completed,
                        *node,
                    )
                );
            }
        }
    }

    Ok(
        ConvergenceExecutionResult::fulfilled(
            plan,
            completed,
        )
    )
    }

pub async fn refresh_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    start: ProjectionNode,
    date: &str,
) -> AppResult<ConvergenceExecutionResult>
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