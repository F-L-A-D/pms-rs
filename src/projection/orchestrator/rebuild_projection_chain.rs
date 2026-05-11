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

        rebuild::{
            guest_summary_rebuild::
                rebuild_guest_summary_projection,

            inventory_projection_rebuild::
                rebuild_inventory_projection,

            reservation_search_rebuild::
                rebuild_reservation_search_projection,

            hotel_inventory_projection_rebuild::
                rebuild_hotel_inventory_projection,
        },

        topology::{
            invalidation_traversal_planner::
                derive_convergence_plan,

            projection_node::
                ProjectionNode,
        },

        orchestrator::
            convergence_execution_result::
                ConvergenceExecutionResult,
    },
};

async fn rebuild_single_projection(
    tx: &mut Transaction<'_, Sqlite>,
    node: ProjectionNode,
) -> AppResult<()>
{
    match node {

        ProjectionNode::GuestSummary => {

            rebuild_guest_summary_projection(
                tx,
            )
            .await?;
        }

        ProjectionNode::ReservationSearch => {

            rebuild_reservation_search_projection(
                tx,
            )
            .await?;
        }

        ProjectionNode::Inventory => {

            rebuild_inventory_projection(
                tx,
            )
            .await?;
        }

        ProjectionNode::HotelInventory => {

            rebuild_hotel_inventory_projection(
                tx,
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn rebuild_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    start: ProjectionNode,
) -> AppResult<ConvergenceExecutionResult>
{
    let plan =
        derive_convergence_plan(
            &ProjectionInvalidation::new(
                start,
                ProjectionScope::Global,
                ProjectionRefreshTarget::Global,
            ),
        );

    let mut completed =
        Vec::new();

    for node in
        plan.convergence_nodes()
    {
        rebuild_single_projection(
            tx,
            *node,
        )
        .await?;

        completed.push(*node);
    }

    Ok(
        ConvergenceExecutionResult::fulfilled(
            plan,
            completed,
        )
    )
}