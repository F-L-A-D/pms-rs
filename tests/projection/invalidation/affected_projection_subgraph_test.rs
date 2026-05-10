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
            invalidation_traversal_plan,

        projection_node::
            ProjectionNode,
    },
};

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
        invalidation_traversal_plan(
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