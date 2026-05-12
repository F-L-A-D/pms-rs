use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{       
        execution::{
            projection_convergence_executor::
                ProjectionConvergenceExecutor,

            projection_execution_registry::
                ProjectionExecutionRegistry,
        },

        invalidation::{
            projection_invalidation::{
                ProjectionInvalidation,
                ProjectionRefreshTarget,
            },

            projection_scope::
                ProjectionScope,
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

    let executor =
        ProjectionExecutionRegistry;

    for step in
        plan.steps()
    {
        executor
            .execute_rebuild(
                tx,
                step.node(),
            )
            .await?;

        completed.push(
            step.node()
        );
    }

    Ok(
        ConvergenceExecutionResult::fulfilled(
            plan,
            completed,
        )
    )
}