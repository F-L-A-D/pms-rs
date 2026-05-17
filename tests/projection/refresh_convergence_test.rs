use serial_test::serial;

use pms_rs::projection::{
    execution::execution_trace::{clear_trace, execution_trace},
    invalidation::{
        projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
        projection_scope::ProjectionScope,
    },
    orchestrator::refresh_projection_chain::refresh_projection_chain,
    topology::projection_node::ProjectionNode,
};

use crate::common::{app::spawn_app, guest::create_guest};

#[tokio::test]
#[serial]
async fn should_execute_refresh_in_topology_order() {
    clear_trace();

    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    let invalidation = ProjectionInvalidation::new(
        ProjectionNode::GuestAggregate,
        ProjectionScope::Global,
        ProjectionRefreshTarget::Guest { guest_id: guest.id },
    );

    let result = refresh_projection_chain(&mut tx, invalidation).await;

    assert!(result.is_ok(),);

    assert_eq!(
        execution_trace(),
        vec![
            ProjectionNode::GuestAggregate,
            ProjectionNode::GuestActivitySignal,
        ],
    );
}
