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

        topology::{
            projection_node::
                ProjectionNode,
                
            invalidation_traversal_planner::
                derive_convergence_plan,
        },

        orchestrator::
            convergence_execution_result::
                ConvergenceExecutionResult,

        execution::{
            projection_convergence_executor::
                ProjectionConvergenceExecutor,

            projection_execution_registry::
                ProjectionExecutionRegistry,
        },
    },
};

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

    let executor = 
        ProjectionExecutionRegistry;

    for step in plan.steps()
    {
        let result =
            executor
                .execute(
                    tx, 
                    step.node(), 
                    &invalidation.target,
                )
                .await;

        match result {

            Ok(_) => {

                completed.push(
                    step.node()
                );
            }

            Err(_error) => {

                return Ok(
                    ConvergenceExecutionResult::failed(
                        plan.clone(),
                        completed,
                        step.node(),
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