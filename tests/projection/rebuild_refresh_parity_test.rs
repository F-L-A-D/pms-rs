use serial_test::serial;

use pms_rs::projection::{
    aggregate::access::fetch_guest_aggregate::fetch_guest_aggregate,
    invalidation::{
        projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
        projection_scope::ProjectionScope,
    },
    orchestrator::{
        rebuild_projection_chain::rebuild_projection_chain,
        refresh_projection_chain::refresh_projection_chain,
    },
    topology::projection_node::ProjectionNode,
};

use crate::common::{app::spawn_app, guest::create_guest};

#[tokio::test]
#[serial]
async fn should_refresh_and_rebuild_returns_same_result() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    let invalidation = ProjectionInvalidation::new(
        ProjectionNode::GuestAggregate,
        ProjectionScope::Global,
        ProjectionRefreshTarget::Guest { guest_id: guest.id },
    );

    refresh_projection_chain(&mut tx, invalidation)
        .await
        .unwrap();

    let refreshed = fetch_guest_aggregate(&mut tx, guest.id).await.unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::GuestAggregate)
        .await
        .unwrap();

    let rebuilt = fetch_guest_aggregate(&mut tx, guest.id).await.unwrap();

    assert_eq!(refreshed, rebuilt,);
}
