use chrono::Utc;

use uuid::Uuid;

use pms_rs::{
    domain::semantic::{
        operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
        operation_context::{OperationActor, OperationSource},
        semantic_activation::{ChangePatternType, SemanticActivationKey},
    },
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::{
            rebuild_projection_chain::rebuild_projection_chain,
            refresh_projection_chain::refresh_projection_chain,
        },
        signal::access::{
            fetch_change_pattern::fetch_change_pattern,
            fetch_confidence_profile::fetch_confidence_profile,
            fetch_semantic_activation::fetch_semantic_activation,
        },
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

use crate::common::app::spawn_app;

#[tokio::test]
async fn should_classify_reservation_operation_semantic_signals() {
    let app = spawn_app().await;
    let mut tx = app.db.begin_tx().await;

    let cases = vec![
        (
            OperationType::Create,
            vec![changed_field("room_class", None, Some("standard"))],
            ChangePatternType::ReservationCreated,
            SemanticActivationKey::ReservationCreated,
        ),
        (
            OperationType::Modify,
            vec![changed_field(
                "check_out",
                Some("2026-05-19"),
                Some("2026-05-20"),
            )],
            ChangePatternType::ReservationDateChanged,
            SemanticActivationKey::InventoryRelevantChange,
        ),
        (
            OperationType::Modify,
            vec![changed_field(
                "room_class",
                Some("standard"),
                Some("deluxe"),
            )],
            ChangePatternType::ReservationRoomClassChanged,
            SemanticActivationKey::InventoryRelevantChange,
        ),
        (
            OperationType::Modify,
            vec![changed_field("daily_details", Some("1"), Some("2"))],
            ChangePatternType::ReservationStayShapeChanged,
            SemanticActivationKey::KpiRelevantChange,
        ),
        (
            OperationType::Modify,
            vec![changed_field(
                "daily_revenue_allocations",
                Some("1"),
                Some("3"),
            )],
            ChangePatternType::ReservationRevenueAllocationChanged,
            SemanticActivationKey::BillingRelevantChange,
        ),
        (
            OperationType::Cancel,
            vec![changed_field(
                "reservation_status",
                Some("confirmed"),
                Some("cancelled"),
            )],
            ChangePatternType::ReservationCancelled,
            SemanticActivationKey::ReservationCancelled,
        ),
    ];

    for (operation_type, changed_fields, expected_pattern, expected_activation) in cases {
        let event = operation_event(operation_type, changed_fields);

        SqliteOperationChangeEventRepository::save(&mut tx, &event)
            .await
            .unwrap();

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::ChangePattern,
                ProjectionScope::Timeline,
                ProjectionRefreshTarget::OperationEvent { event_id: event.id },
            ),
        )
        .await
        .unwrap();

        let pattern = fetch_change_pattern(&mut tx, event.id)
            .await
            .unwrap()
            .unwrap();
        let activation = fetch_semantic_activation(&mut tx, event.id)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(pattern.pattern_type, expected_pattern);
        assert_eq!(activation.activation_key, expected_activation);
    }

    let _ = tx.rollback().await;
}

#[tokio::test]
async fn should_rebuild_semantic_signal_chain_equivalent_to_refresh() {
    let app = spawn_app().await;
    let mut tx = app.db.begin_tx().await;

    let event = operation_event(
        OperationType::Modify,
        vec![changed_field("room_class", Some("standard"), Some("suite"))],
    );

    SqliteOperationChangeEventRepository::save(&mut tx, &event)
        .await
        .unwrap();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::ChangePattern,
            ProjectionScope::Timeline,
            ProjectionRefreshTarget::OperationEvent { event_id: event.id },
        ),
    )
    .await
    .unwrap();

    let refreshed_pattern = fetch_change_pattern(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();
    let refreshed_confidence = fetch_confidence_profile(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();
    let refreshed_activation = fetch_semantic_activation(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();

    let rebuild = rebuild_projection_chain(&mut tx, ProjectionNode::ChangePattern)
        .await
        .unwrap();

    assert!(rebuild.convergence_fulfilled());

    let rebuilt_pattern = fetch_change_pattern(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();
    let rebuilt_confidence = fetch_confidence_profile(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();
    let rebuilt_activation = fetch_semantic_activation(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(refreshed_pattern.pattern_type, rebuilt_pattern.pattern_type);
    assert_eq!(
        refreshed_pattern.operation_type,
        rebuilt_pattern.operation_type
    );
    assert_eq!(
        refreshed_pattern.changed_fields_json,
        rebuilt_pattern.changed_fields_json
    );
    assert_eq!(
        refreshed_confidence.confidence_score,
        rebuilt_confidence.confidence_score
    );
    assert_eq!(
        refreshed_confidence.reasons_json,
        rebuilt_confidence.reasons_json
    );
    assert_eq!(
        refreshed_activation.activation_key,
        rebuilt_activation.activation_key
    );
    assert_eq!(
        refreshed_activation.activation_score,
        rebuilt_activation.activation_score
    );
    assert_eq!(
        refreshed_activation.confidence_score,
        rebuilt_activation.confidence_score
    );
    assert_eq!(refreshed_activation.is_active, rebuilt_activation.is_active);

    let _ = tx.rollback().await;
}

fn operation_event(
    operation_type: OperationType,
    changed_fields: Vec<ChangedField>,
) -> OperationChangeEvent {
    OperationChangeEvent {
        id: Uuid::new_v4(),
        operation_id: Uuid::new_v4(),
        aggregate_type: "reservation".to_string(),
        aggregate_id: Uuid::new_v4(),
        operation_type,
        actor: OperationActor::System,
        actor_id: None,
        source: OperationSource::Api,
        before_json: match operation_type {
            OperationType::Create => None,
            OperationType::Modify 
            | OperationType::Cancel
            | OperationType::NoShow
                 => {
                    Some(serde_json::json!({"reservation_status": "confirmed"}).to_string())
                }
            OperationType::Reinstate 
                => Some(serde_json::json!({"reservation_status": "canceled"}).to_string())
        },
        after_json: serde_json::json!({"reservation_status": "confirmed"}).to_string(),
        changed_fields_json: serde_json::to_string(&changed_fields).unwrap(),
        occurred_at: Utc::now(),
    }
}

fn changed_field(
    field_name: &str,
    before_value: Option<&str>,
    after_value: Option<&str>,
) -> ChangedField {
    ChangedField::new(
        field_name,
        before_value.map(ToString::to_string),
        after_value.map(ToString::to_string),
    )
}
