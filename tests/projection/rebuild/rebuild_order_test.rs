use pms_rs::projection::topology::{
    projection_node::
        ProjectionNode,

    projection_ordering::
        rebuild_order,
};

#[test]
fn should_build_rebuild_order_without_invalidation_semantics()
{
    let order =
        rebuild_order(
            ProjectionNode::Inventory
        );

    assert_eq!(
        order,

        vec![
            ProjectionNode::Inventory,
            ProjectionNode::HotelInventory,
        ]
    );
}