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
fn should_create_invalidation_traversal_plan()
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
        plan.scope,
        ProjectionScope::Inventory,
    );
}