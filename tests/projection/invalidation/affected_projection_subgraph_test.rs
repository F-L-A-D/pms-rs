use pms_rs::projection::{
    invalidation::{
        affected_projection_subgraph::
            AffectedProjectionSubgraph,

        projection_scope::
            ProjectionScope,

        projection_invalidation::{
            ProjectionInvalidation,
            ProjectionRefreshTarget,
        },
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

        invalidation_traversal_planner::
            derive_convergence_plan,
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
fn should_build_affected_projection_subgraph()
{
    let invalidation =
        ProjectionInvalidation::new(
            ProjectionNode::Inventory,

            ProjectionScope::Inventory,

            ProjectionRefreshTarget::InventoryDate {
                date:
                    "2026-05-10"
                        .to_string(),
            },
        );

    let plan =
        derive_convergence_plan(
            &invalidation
        );

    assert_eq!(
        plan
            .affected_subgraph
            .nodes,

        vec![
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ]
    );

    assert_eq!(
        plan
            .affected_subgraph
            .dependencies
            .len(),

        1,
    );

    assert_eq!(
        plan
            .affected_subgraph
            .dependencies[0]
            .upstream,

        ProjectionNode::Inventory,
    );

    assert_eq!(
        plan
            .affected_subgraph
            .dependencies[0]
            .downstream,

        ProjectionNode::HotelInventory,
    );
}

#[test]
fn should_mark_aborted_convergence_execution()
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

    assert_eq!(
        result.status,
        ConvergenceExecutionStatus::Aborted,
    );

    assert!(
        !result.convergence_fulfilled()
    );
}

#[test]
fn should_allow_resume_after_abort()
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
        result.resumable()
    );

    assert_eq!(
        result.remaining_nodes(),

        vec![
            ProjectionNode::HotelInventory,
        ],
    );
}

#[test]
fn should_not_expose_aborted_convergence_as_visible()
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
        !result.authoritatively_visible()
    );

    assert!(
        result
            .aborted_before_boundary_visibility()
    );
}