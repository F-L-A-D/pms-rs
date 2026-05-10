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
        projection_node::
            ProjectionNode,

        invalidation_traversal_planner::
            invalidation_traversal_plan,
    },
};

#[test]
fn should_propagate_when_scope_matches()
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
}

#[test]
fn should_not_propagate_when_scope_mismatches()
{
    let invalidation =
        ProjectionInvalidation::new(
            ProjectionNode::Inventory,

            ProjectionScope::Guest,

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
        ]
    );
}