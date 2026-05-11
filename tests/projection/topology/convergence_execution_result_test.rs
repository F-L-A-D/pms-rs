use pms_rs::projection::{
    invalidation::{
        affected_projection_subgraph::
            AffectedProjectionSubgraph,

        projection_scope::
            ProjectionScope,
    },

    orchestrator::
        convergence_execution_result::{
            ConvergenceExecutionResult,
            ConvergenceExecutionStatus,
        },

    topology::{
        convergence_traversal_plan::
            ConvergenceTraversalPlan,

        projection_node::
            ProjectionNode,
    },
};

fn build_plan() -> ConvergenceTraversalPlan
{
    ConvergenceTraversalPlan::new(
        AffectedProjectionSubgraph::new(
            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],

            vec![],
        ),

        vec![
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ],

        ProjectionScope::Inventory,
    )
}

#[test]
fn should_mark_convergence_as_fulfilled_when_all_nodes_complete()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::fulfilled(
            plan,

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],
        );

    assert!(
        result
            .convergence_fulfilled()
    );

    assert!(
        result
            .completed_all_nodes()
    );

    assert_eq!(
        result.status,
        ConvergenceExecutionStatus::Fulfilled,
    );
}

#[test]
fn should_not_mark_convergence_as_complete_when_nodes_missing()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::fulfilled(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],
        );

    assert!(
        result
            .convergence_fulfilled()
    );

    assert!(
        !result
            .completed_all_nodes()
    );
}

#[test]
fn should_detect_partial_completion()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .partially_completed()
    );

    assert!(
        !result
            .authoritative_boundary_fulfilled()
    );
}

#[test]
fn should_preserve_boundary_atomicity_after_fulfilled_convergence()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::fulfilled(
            plan,

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],
        );

    assert!(
        result
            .preserves_boundary_atomicity()
    );
}

#[test]
fn should_not_expose_failed_convergence_as_visible()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        !result
            .authoritatively_visible()
    );

    assert!(
        !result
            .preserves_boundary_atomicity()
    );
}

#[test]
fn should_allow_resume_after_partial_failure()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .resumable()
    );

    assert_eq!(
        result.remaining_nodes(),

        vec![
            ProjectionNode::HotelInventory,
        ],
    );

    assert!(
        result
            .preserves_resume_boundary()
    );
}

#[test]
fn should_not_allow_resume_after_fulfilled_convergence()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::fulfilled(
            plan,

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],
        );

    assert!(
        !result
            .resumable()
    );

    assert!(
        result
            .remaining_nodes()
            .is_empty()
    );
}

#[test]
fn should_not_mark_duplicate_completion_as_full_convergence()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::fulfilled(
            plan,

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::Inventory,
            ],
        );

    assert!(
        !result.completed_all_nodes()
    );
}

#[test]
fn should_preserve_checkpoint_boundary_inside_plan()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .preserves_checkpoint_boundary()
    );
}

#[test]
fn should_reject_checkpointability_by_default()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        !result.checkpointable()
    );
}

#[test]
fn should_classify_projection_execution_failure()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .failed_due_to_projection_execution()
    );
}

#[test]
fn should_preserve_authoritative_isolation_after_abort()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::aborted(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],
        );

    assert!(
        result
            .preserves_authoritative_isolation()
    );
}

#[test]
fn should_preserve_authoritative_isolation_after_partial_failure()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .preserves_authoritative_isolation()
    );
}

#[test]
fn should_record_failed_projection_node()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .convergence_failed_at(
                ProjectionNode::HotelInventory
            )
    );
}

#[test]
fn should_preserve_failed_node_isolation()
{
    let plan =
        build_plan();

    let result =
        ConvergenceExecutionResult::failed(
            plan,

            vec![
                ProjectionNode::Inventory,
            ],

            ProjectionNode::HotelInventory,
        );

    assert!(
        result
            .preserves_failure_isolation()
    );
}   