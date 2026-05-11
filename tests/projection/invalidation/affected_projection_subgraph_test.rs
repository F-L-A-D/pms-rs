use pms_rs::projection::{
    invalidation::{
        projection_invalidation::{
            ProjectionInvalidation,
            ProjectionRefreshTarget,
        },

        projection_scope::
            ProjectionScope,

        affected_projection_subgraph::
            AffectedProjectionSubgraph,
    },

    topology::{
        invalidation_traversal_planner::
            invalidation_traversal_plan,

        projection_node::
            ProjectionNode,
        
        projection_dependency::
              ProjectionDependency,
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

#[test]
fn should_expose_boundary_nodes_as_rebuild_responsibility()
{
    let subgraph =
        AffectedProjectionSubgraph::new(

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],

            vec![],
        );

    assert_eq!(
        subgraph
            .rebuild_boundary_nodes(),

        &[
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ]
    );
}

#[test]
fn should_require_authoritative_rebuild_convergence()
{
    let subgraph =
        AffectedProjectionSubgraph::new(

            vec![
                ProjectionNode::Inventory,
                ProjectionNode::HotelInventory,
            ],

            vec![],
        );

    assert!(
        subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::Inventory
            )
    );

    assert!(
        subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::HotelInventory
            )
    );
}

#[test]
fn should_execute_refresh_only_for_nodes_requiring_rebuild_equivalence()
{
    let subgraph =
        AffectedProjectionSubgraph::new(

            vec![
                ProjectionNode::Inventory,
            ],

            vec![],
        );

    assert!(
        subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::Inventory
            )
    );

    assert!(
        !subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::HotelInventory
            )
    );
}

#[test]
fn should_treat_authoritative_convergence_as_boundary_contract()
{
    let subgraph =
        AffectedProjectionSubgraph::new(

            vec![
                ProjectionNode::Inventory,
            ],

            vec![],
        );

    assert!(
        subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::Inventory
            )
    );

    assert!(
        !subgraph
            .must_converge_to_authoritative_rebuild(
                ProjectionNode::HotelInventory
            )
    );
}