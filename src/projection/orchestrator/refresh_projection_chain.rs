use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::projection_convergence_executor::ProjectionConvergenceExecutor,
        invalidation::projection_invalidation::ProjectionInvalidation,
        orchestrator::convergence_execution_result::ConvergenceExecutionResult,
        topology::invalidation_traversal_planner::derive_convergence_plan,
    },
};

pub async fn propagate_invalidation(
    tx: &mut Transaction<'_, Sqlite>,
    invalidation: ProjectionInvalidation,
) -> AppResult<ConvergenceExecutionResult> {
    let plan = derive_convergence_plan(&invalidation);

    println!("runtime plan: {:?}", plan.convergence_nodes(),);

    let mut completed = Vec::new();

    for step in plan.steps() {
        println!("executing node: {:?}", step.node(),);

        let result =
            ProjectionConvergenceExecutor::execute_refresh(tx, &step.node(), &invalidation.target)
                .await;

        match result {
            Ok(_) => {
                completed.push(step.node());
            }

            Err(error) => {
                println!("{:#?}", error);

                return Ok(ConvergenceExecutionResult::failed(
                    plan.clone(),
                    completed,
                    step.node(),
                ));
            }
        }
    }

    Ok(ConvergenceExecutionResult::fulfilled(plan, completed))
}

pub async fn refresh_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    invalidation: ProjectionInvalidation,
) -> AppResult<ConvergenceExecutionResult> {
    propagate_invalidation(tx, invalidation).await
}
