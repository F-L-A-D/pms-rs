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
        plan.convergence_nodes(),

        &[
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ]
    );
}

#[test]
fn should_include_authoritative_boundary_in_convergence_plan()
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

    assert!(
        plan
            .affected_subgraph
            .contains_node(
                ProjectionNode::Inventory
            )
    );

    assert!(
        plan
            .affected_subgraph
            .contains_node(
                ProjectionNode::HotelInventory
            )
    );
}

#[test]
fn should_preserve_topology_ordering_in_convergence_plan()
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
        plan.convergence_nodes()[0],
        ProjectionNode::Inventory,
    );

    assert_eq!(
        plan.convergence_nodes()[1],
        ProjectionNode::HotelInventory,
    );
}