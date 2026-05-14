use pms_rs::projection::{
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
};

#[test]
fn should_derive_deterministic_convergence_order()
{
    let invalidation =
        ProjectionInvalidation::new(
            ProjectionNode::GuestAggregate,
            ProjectionScope::Global,
            ProjectionRefreshTarget::Global,
        );

    let first =
        derive_convergence_plan(
            &invalidation,
        );

    let second =
        derive_convergence_plan(
            &invalidation,
        );

    assert_eq!(
        first.convergence_nodes(),
        second.convergence_nodes(),
    );

    assert_eq!(
        first.convergence_nodes(),

        vec![
            ProjectionNode::GuestAggregate,
            ProjectionNode::GuestActivitySignal,
        ],
    );
}