use pms_rs::projection::{
    invalidation::{
        projection_invalidation::
            ProjectionInvalidation,

        projection_scope::
            ProjectionScope,
    },

    topology::{
        projection_node::
            ProjectionNode,

        projection_ordering::
            invalidation_traversal_plan,
    },
};

#[test]
fn should_create_invalidation_traversal_plan()
{
    let invalidation =
        ProjectionInvalidation::new(
            ProjectionNode::Inventory,

            ProjectionScope::Date {
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
        plan.ordered_nodes,

        vec![
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ]
    );

    assert_eq!(
        plan.scope,

        ProjectionScope::Date {
            date:
                "2026-05-10"
                    .to_string(),
        }
    );
}