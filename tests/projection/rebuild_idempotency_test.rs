use serial_test::serial;

use pms_rs::projection::{
    aggregate::access::fetch_guest_aggregate::fetch_guest_aggregate,
    orchestrator::rebuild_projection_chain::rebuild_projection_chain,
    topology::projection_node::ProjectionNode,
};

use crate::common::{app::spawn_app, guest::create_guest};

#[tokio::test]
#[serial]
async fn should_rebuild_is_idempotent() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    rebuild_projection_chain(&mut tx, ProjectionNode::GuestAggregate)
        .await
        .unwrap();

    let first = fetch_guest_aggregate(&mut tx, guest.id).await.unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::GuestAggregate)
        .await
        .unwrap();

    let second = fetch_guest_aggregate(&mut tx, guest.id).await.unwrap();

    assert_eq!(first, second,);
}
