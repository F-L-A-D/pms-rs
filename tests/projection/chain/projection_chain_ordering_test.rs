use pms_rs::projection::topology::{
    projection_node::
        ProjectionNode,

    projection_ordering::{
        downstream_of,
        rebuild_order,
    },
};

#[test]
fn should_return_downstream_projection()
{
    let downstream =
        downstream_of(
            ProjectionNode::Inventory
        );

    assert_eq!(
        downstream,
        vec![
            ProjectionNode::HotelInventory
        ]
    );
}

#[test]
fn should_return_rebuild_order()
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