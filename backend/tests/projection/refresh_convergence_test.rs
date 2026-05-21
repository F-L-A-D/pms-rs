use serial_test::serial;

use chrono::Utc;

use uuid::Uuid;

use pms_rs::{
    domain::semantic::{
        operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
        operation_context::{OperationActor, OperationSource},
    },
    projection::{
        execution::execution_trace::{clear_trace, execution_trace},
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
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

#[tokio::test]
#[serial]
async fn should_execute_semantic_activation_flow_in_topology_order() {
    clear_trace();

    let app = spawn_app().await;
    let mut tx = app.db.begin_tx().await;
    let event_id = Uuid::new_v4();

    SqliteOperationChangeEventRepository::save(
        &mut tx,
        &OperationChangeEvent {
            id: event_id,
            operation_id: Uuid::new_v4(),
            aggregate_type: "reservation".to_string(),
            aggregate_id: Uuid::new_v4(),
            operation_type: OperationType::Modify,
            actor: OperationActor::System,
            actor_id: None,
            source: OperationSource::Api,
            before_json: Some(serde_json::json!({"room_class": "standard"}).to_string()),
            after_json: serde_json::json!({"room_class": "deluxe"}).to_string(),
            changed_fields_json: serde_json::to_string(&vec![ChangedField::new(
                "room_class",
                Some("standard".to_string()),
                Some("deluxe".to_string()),
            )])
            .unwrap(),
            occurred_at: Utc::now(),
        },
    )
    .await
    .unwrap();

    let invalidation = ProjectionInvalidation::new(
        ProjectionNode::ChangePattern,
        ProjectionScope::Timeline,
        ProjectionRefreshTarget::OperationEvent { event_id },
    );

    let result = refresh_projection_chain(&mut tx, invalidation).await;

    assert!(result.is_ok());

    assert_eq!(
        execution_trace(),
        vec![
            ProjectionNode::ChangePattern,
            ProjectionNode::ConfidenceProfile,
            ProjectionNode::SemanticActivation,
        ],
    );

    let _ = tx.rollback().await;
}
