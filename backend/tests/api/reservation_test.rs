use axum::http::StatusCode;

use chrono::{Duration, Utc};

use uuid::Uuid;

use pms_rs::{
    api::dto::response::reservation::ReservationResponse,
    domain::{
        entity::reservation::{ReservationStatus, StayStatus},
        reservation_guest_relation::ReservationGuestRelationType,
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::{OperationActor, OperationSource},
            reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
            reservation_transition::ReservationTransitionType,
            semantic_activation::SemanticActivationKey,
        },
    },
    projection::signal::access::fetch_semantic_activation::fetch_semantic_activation,
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
            reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
            reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
        },
    },
};

use crate::common::{
    app::spawn_app,
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{delete, delete_json, get, patch_json, post_json, response_json},
    guest::create_guest,
    reservation::{create_reservation, create_reservation_with_guest},
    room::create_room,
    stay::assign_room,
};

#[tokio::test]
async fn should_roundtrip_reservation() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let response = get(&app.app, &format!("/reservations/{}", created.id,)).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let retrieved: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(retrieved.id, created.id,);

    assert_eq!(retrieved.room_class, created.room_class,);

    assert_eq!(retrieved.check_in, created.check_in,);

    assert_eq!(retrieved.check_out, created.check_out,);

    assert_eq!(retrieved.participants.len(), 1,);

    assert_eq!(
        retrieved.participants[0].guest_id,
        created.participants[0].guest_id,
    );
}

#[tokio::test]
async fn should_return_reservation_detail_visibility_fields() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let reservation = create_reservation_with_guest(&app.app, guest.id).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let edit_session_response = post_json(
        &app.app,
        &format!("/reservations/{}/edit-sessions", reservation.id),
        &serde_json::json!({
            "actor_id": "front-1",
            "actor_label": "Front 1"
        }),
    )
    .await;

    assert_eq!(edit_session_response.status(), StatusCode::OK);

    let response = get(&app.app, &format!("/reservations/{}", reservation.id)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let detail: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(detail.id, reservation.id);
    assert_eq!(detail.operation_metadata.version, 2);
    assert_eq!(detail.room_assignment.room_id, Some(room.id));

    let assigned_room = detail.room_assignment.room.unwrap();
    assert_eq!(assigned_room.room_no, room.room_no);
    assert_eq!(assigned_room.room_class, room.room_class);

    assert_eq!(detail.participant_details.len(), 1);
    assert_eq!(detail.participant_details[0].guest_id, guest.id);

    let participant_guest = detail.participant_details[0].guest.as_ref().unwrap();
    assert_eq!(participant_guest.last_name, guest.last_name);
    assert_eq!(participant_guest.first_name, guest.first_name);
    assert_eq!(detail.active_edit_sessions.len(), 1);
    assert_eq!(detail.active_edit_sessions[0].actor_id, "front-1");
    assert_eq!(detail.room_history.len(), 1);
    assert_eq!(detail.room_history[0].after_room_id, Some(room.id));
}

#[tokio::test]
async fn should_return_sleep_sharing_children_and_reservation_notes() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let today = Utc::now().date_naive();

    let create_response = post_json(
        &app.app,
        "/reservations",
        &serde_json::json!({
            "check_in": today.to_string(),
            "check_out": (today + Duration::days(1)).to_string(),
            "room_class": "standard",
            "daily_details": [
                {
                    "service_date": today.to_string(),
                    "room_class": "standard",
                    "adult_count": 2,
                    "child_count": 1,
                    "sleep_sharing_child_count": 1,
                    "sleep_sharing_children": [
                        {
                            "name": "Sleep Share Child",
                            "age": 4,
                            "gender": "unspecified"
                        }
                    ]
                }
            ],
            "participants": [
                {
                    "guest_id": guest.id.to_string(),
                    "relation_type": "primary"
                }
            ]
        }),
    )
    .await;

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    let memo_response = post_json(
        &app.app,
        &format!("/reservations/{}/notes", created.id),
        &serde_json::json!({
            "kind": "global_memo",
            "body": "Prefers quiet room",
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(memo_response.status(), StatusCode::CREATED);

    let trace_response = post_json(
        &app.app,
        &format!("/reservations/{}/traces", created.id),
        &serde_json::json!({
            "department_code": "hk",
            "body": "Prepare extra towels",
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(trace_response.status(), StatusCode::CREATED);

    let detail_response = get(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(detail_response.status(), StatusCode::OK);

    let detail: ReservationResponse =
        serde_json::from_value(response_json(detail_response).await).unwrap();

    assert_eq!(detail.daily_details[0].adult_count, 2);
    assert_eq!(detail.daily_details[0].child_count, 1);
    assert_eq!(detail.daily_details[0].sleep_sharing_child_count, 1);
    assert_eq!(detail.daily_details[0].sleep_sharing_children.len(), 1);
    assert_eq!(
        detail.daily_details[0].sleep_sharing_children[0]
            .name
            .as_deref(),
        Some("Sleep Share Child")
    );
    assert_eq!(
        detail.daily_details[0].sleep_sharing_children[0].age,
        Some(4)
    );

    assert_eq!(detail.notes.len(), 1);
    assert_eq!(detail.traces.len(), 1);
    assert!(detail
        .notes
        .iter()
        .any(|note| note.body == "Prefers quiet room"));
    assert!(detail.traces.iter().any(|note| {
        note.department_code.as_deref() == Some("hk") && note.body == "Prepare extra towels"
    }));
    assert!(!detail.operation_events.is_empty());
    assert!(detail
        .operation_events
        .iter()
        .any(|event| event.operation_type == OperationType::Create));
    assert!(detail
        .audit_logs
        .iter()
        .any(|log| log.action == "reservation.memo.add"));
    assert!(detail
        .audit_logs
        .iter()
        .any(|log| log.action == "reservation.trace.add"));
}

#[tokio::test]
async fn should_reject_stale_reservation_update_and_allow_retry_with_latest_version() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    assert_eq!(created.version, 1);

    let staff_a_view = get(&app.app, &format!("/reservations/{}", created.id)).await;
    let staff_b_view = get(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(staff_a_view.status(), StatusCode::OK);
    assert_eq!(staff_b_view.status(), StatusCode::OK);

    let staff_a_reservation: ReservationResponse =
        serde_json::from_value(response_json(staff_a_view).await).unwrap();
    let staff_b_reservation: ReservationResponse =
        serde_json::from_value(response_json(staff_b_view).await).unwrap();

    assert_eq!(staff_a_reservation.version, staff_b_reservation.version);

    let staff_a_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &serde_json::json!({
            "expected_version": staff_a_reservation.version,
            "check_out": (created.check_out + Duration::days(1)).to_string()
        }),
    )
    .await;

    assert_eq!(staff_a_response.status(), StatusCode::OK);

    let staff_a_updated: ReservationResponse =
        serde_json::from_value(response_json(staff_a_response).await).unwrap();

    assert_eq!(staff_a_updated.version, staff_a_reservation.version + 1);

    let mut tx = app.db.begin_tx().await;

    let event_count_after_staff_a = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap()
        .into_iter()
        .filter(|event| event.aggregate_id == created.id)
        .count();

    let _ = tx.rollback().await;

    let stale_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &serde_json::json!({
            "expected_version": staff_b_reservation.version,
            "room_class": "deluxe"
        }),
    )
    .await;

    assert_eq!(stale_response.status(), StatusCode::CONFLICT);

    let mut tx = app.db.begin_tx().await;

    let event_count_after_stale = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap()
        .into_iter()
        .filter(|event| event.aggregate_id == created.id)
        .count();

    let _ = tx.rollback().await;

    assert_eq!(event_count_after_stale, event_count_after_staff_a);

    let latest_response = get(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(latest_response.status(), StatusCode::OK);

    let latest: ReservationResponse =
        serde_json::from_value(response_json(latest_response).await).unwrap();

    assert_eq!(latest.version, staff_a_updated.version);
    assert_eq!(latest.check_out, created.check_out + Duration::days(1));

    let retry_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &serde_json::json!({
            "expected_version": latest.version,
            "room_class": "deluxe"
        }),
    )
    .await;

    assert_eq!(retry_response.status(), StatusCode::OK);

    let retry_updated: ReservationResponse =
        serde_json::from_value(response_json(retry_response).await).unwrap();

    assert_eq!(retry_updated.version, latest.version + 1);
    assert_eq!(retry_updated.check_out, latest.check_out);
    assert_eq!(retry_updated.room_class, "deluxe");
}

#[tokio::test]
async fn should_warn_when_reservation_is_already_open_for_editing_without_read_lock() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let read_response = get(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(read_response.status(), StatusCode::OK);

    let staff_a_response = post_json(
        &app.app,
        &format!("/reservations/{}/edit-sessions", created.id),
        &serde_json::json!({
            "actor_id": "staff-a",
            "actor_label": "Staff A"
        }),
    )
    .await;

    assert_eq!(staff_a_response.status(), StatusCode::OK);

    let staff_a_body = response_json(staff_a_response).await;

    assert!(staff_a_body["warning"].is_null());

    let staff_b_response = post_json(
        &app.app,
        &format!("/reservations/{}/edit-sessions", created.id),
        &serde_json::json!({
            "actor_id": "staff-b",
            "actor_label": "Staff B"
        }),
    )
    .await;

    assert_eq!(staff_b_response.status(), StatusCode::OK);

    let staff_b_body = response_json(staff_b_response).await;

    let active_sessions = staff_b_body["warning"]["active_sessions"]
        .as_array()
        .unwrap();

    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0]["actor_id"], "staff-a");
}

#[tokio::test]
async fn should_reject_invalid_stay_range() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let participant = ReservationParticipantBuilder::new(guest.id).build();

    let today = Utc::now().date_naive();

    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(today + Duration::days(2))
        .with_check_out(today)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_empty_participants() {
    let app = spawn_app().await;

    let request = ReservationBuilder::new().build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_multiple_primary_participants() {
    let app = spawn_app().await;

    let guest1 = create_guest(&app.app).await;

    let guest2 = create_guest(&app.app).await;

    let participant1 = ReservationParticipantBuilder::new(guest1.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let participant2 = ReservationParticipantBuilder::new(guest2.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let request = ReservationBuilder::new()
        .with_participant(participant1)
        .with_participant(participant2)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_duplicate_participant_guest_ids() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let participant1 = ReservationParticipantBuilder::new(guest.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let participant2 = ReservationParticipantBuilder::new(guest.id)
        .with_relation_type(ReservationGuestRelationType::Accompany)
        .build();

    let request = ReservationBuilder::new()
        .with_participant(participant1)
        .with_participant(participant2)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_create_reservation_with_booking_channel_and_package_breakdowns() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "external_id": "RES-PACKAGE-001",
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(1)).to_string(),
        "room_class": "standard",
        "booking_channel": "ota",
        "plan_code": "BB",
        "package_breakdowns": [
            {
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "120.00"
            },
            {
                "package_code": "BREAKFAST",
                "revenue_category": "food_and_beverage",
                "amount": "30.00"
            },
            {
                "package_code": "TAX",
                "revenue_category": "tax",
                "amount": "15.00"
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(created.booking_channel, ReservationBookingChannel::Ota);
    assert_eq!(created.plan_code.as_deref(), Some("BB"));
    assert_eq!(created.package_breakdowns.len(), 3);
    assert!(created.package_breakdowns.iter().any(|breakdown| {
        breakdown.package_code == "ROOM"
            && breakdown.revenue_category == ReservationRevenueCategory::Room
            && breakdown.amount == rust_decimal::Decimal::new(12000, 2)
    }));

    let mut tx = app.db.begin_tx().await;

    let breakdowns =
        SqliteReservationPackageBreakdownRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(breakdowns.len(), 3);
    assert!(breakdowns.iter().any(|breakdown| {
        breakdown.package_code == "BREAKFAST"
            && breakdown.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && breakdown.amount == rust_decimal::Decimal::new(3000, 2)
    }));
}

#[tokio::test]
async fn should_record_operation_change_event_and_activation_on_reservation_create() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();
    let event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Create
        })
        .unwrap();

    assert_eq!(event.aggregate_type, "reservation");
    assert_eq!(event.actor, OperationActor::System);
    assert_eq!(event.source, OperationSource::Api);
    assert!(event.actor_id.is_none());
    assert!(event.before_json.is_none());
    assert!(event.after_json.contains(created.id.to_string().as_str()));
    let changed_fields: Vec<ChangedField> =
        serde_json::from_str(&event.changed_fields_json).unwrap();
    let room_class_change = changed_fields
        .iter()
        .find(|field| field.field_name == "room_class")
        .unwrap();
    assert!(room_class_change.before_value.is_none());
    assert_eq!(
        room_class_change.after_value.as_deref(),
        Some(created.room_class.as_str())
    );

    let activation = fetch_semantic_activation(&mut tx, event.id)
        .await
        .unwrap()
        .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(
        activation.activation_key,
        SemanticActivationKey::ReservationCreated
    );
    assert!(activation.is_active);
}

#[tokio::test]
async fn should_record_operational_audit_logs_for_reservation_workflow() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let modify_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &serde_json::json!({
            "expected_version": created.version,
            "check_out": (created.check_out + Duration::days(1)).to_string()
        }),
    )
    .await;

    assert_eq!(modify_response.status(), StatusCode::OK);

    let modified: ReservationResponse =
        serde_json::from_value(response_json(modify_response).await).unwrap();

    let stale_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &serde_json::json!({
            "expected_version": created.version,
            "room_class": "deluxe"
        }),
    )
    .await;

    assert_eq!(stale_response.status(), StatusCode::CONFLICT);

    let cancel_response = delete(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(cancel_response.status(), StatusCode::OK);

    let response = get(&app.app, &format!("/audit-logs/reservation/{}", created.id)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    let logs = body.as_array().unwrap();

    assert_eq!(logs.len(), 3);
    assert!(logs.iter().any(|log| log["action"] == "reservation.create"));
    assert!(logs.iter().any(|log| log["action"] == "reservation.modify"));
    assert!(logs.iter().any(|log| log["action"] == "reservation.cancel"));
    assert!(!logs
        .iter()
        .any(|log| log["after_json"].as_str().unwrap().contains("deluxe")));

    let modify_log = logs
        .iter()
        .find(|log| log["action"] == "reservation.modify")
        .unwrap();

    assert!(modify_log["changed_fields_json"]
        .as_str()
        .unwrap()
        .contains("check_out"));
    assert!(modify_log["after_json"]
        .as_str()
        .unwrap()
        .contains(&modified.version.to_string()));
}

#[tokio::test]
async fn should_fetch_operation_semantic_signal_for_reservation_change_event() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();
    let event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Create
        })
        .unwrap();
    let event_id = event.id;

    let _ = tx.rollback().await;

    let response = get(
        &app.app,
        &format!("/operation-events/{}/semantic-signal", event_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;

    assert_eq!(body["event_id"], event_id.to_string());
    assert_eq!(
        body["change_pattern"]["pattern_type"],
        "reservation_created"
    );
    assert_eq!(
        body["confidence_profile"]["reasons"][0]["rule"],
        "fixed_operation_type_baseline"
    );
    assert_eq!(
        body["semantic_activation"]["activation_key"],
        "reservation_created"
    );
    assert!(body["change_pattern"]["changed_fields"]
        .as_array()
        .unwrap()
        .iter()
        .any(|field| field["field_name"] == "room_class"));
}

#[tokio::test]
async fn should_return_not_found_for_missing_operation_semantic_signal_event() {
    let app = spawn_app().await;

    let response = get(
        &app.app,
        &format!("/operation-events/{}/semantic-signal", Uuid::new_v4()),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn should_fetch_operation_semantic_signal_with_missing_projection_rows() {
    let app = spawn_app().await;
    let event_id = Uuid::new_v4();

    let mut tx = app.db.begin_tx().await;

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

    tx.commit().await.unwrap();

    let response = get(
        &app.app,
        &format!("/operation-events/{}/semantic-signal", event_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;

    assert_eq!(body["event_id"], event_id.to_string());
    assert!(body["change_pattern"].is_null());
    assert!(body["confidence_profile"].is_null());
    assert!(body["semantic_activation"].is_null());
}

#[tokio::test]
async fn should_create_reservation_with_daily_stay_details_and_daily_revenue_allocations() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "external_id": "RES-DAILY-001",
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(2)).to_string(),
        "room_class": "standard",
        "booking_channel": "direct",
        "plan_code": "BASE",
        "daily_details": [
            {
                "service_date": today.to_string(),
                "room_class": "standard",
                "plan_code": "BB",
                "adult_count": 1,
                "child_count": 0,
                "package_breakdowns": [
                    {
                        "package_code": "ROOM",
                        "revenue_category": "room",
                        "amount": "100.00"
                    },
                    {
                        "package_code": "TAX",
                        "revenue_category": "tax",
                        "amount": "10.00"
                    }
                ]
            },
            {
                "service_date": (today + Duration::days(1)).to_string(),
                "room_class": "deluxe",
                "plan_code": "HB",
                "adult_count": 2,
                "child_count": 1,
                "package_breakdowns": [
                    {
                        "package_code": "ROOM",
                        "revenue_category": "room",
                        "amount": "180.00"
                    },
                    {
                        "package_code": "DINNER",
                        "revenue_category": "food_and_beverage",
                        "amount": "40.00"
                    },
                    {
                        "package_code": "TAX",
                        "revenue_category": "tax",
                        "amount": "20.00"
                    }
                ]
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(created.daily_details.len(), 2);
    assert!(created.daily_details.iter().any(|detail| {
        detail.service_date == today
            && detail.room_class == "standard"
            && detail.plan_code.as_deref() == Some("BB")
            && detail.adult_count == 1
            && detail.child_count == 0
    }));
    assert!(created.daily_details.iter().any(|detail| {
        detail.service_date == today + Duration::days(1)
            && detail.room_class == "deluxe"
            && detail.plan_code.as_deref() == Some("HB")
            && detail.adult_count == 2
            && detail.child_count == 1
    }));
    assert_eq!(created.daily_revenue_allocations.len(), 5);

    let mut tx = app.db.begin_tx().await;

    let details =
        SqliteReservationDailyStayDetailRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();
    let allocations = SqliteReservationDailyRevenueAllocationRepository::list_by_reservation_id(
        &mut tx, created.id,
    )
    .await
    .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(details.len(), 2);
    assert_eq!(allocations.len(), 5);
    assert!(allocations.iter().any(|allocation| {
        allocation.service_date == today + Duration::days(1)
            && allocation.package_code == "DINNER"
            && allocation.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && allocation.amount == rust_decimal::Decimal::new(4000, 2)
    }));
}

#[tokio::test]
async fn should_reject_daily_details_that_do_not_cover_every_reservation_night() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(2)).to_string(),
        "room_class": "standard",
        "daily_details": [
            {
                "service_date": today.to_string(),
                "room_class": "standard",
                "adult_count": 1,
                "child_count": 0
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_modify_reservation_daily_details_and_revenue_allocations() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;
    let next_check_out = created.check_out + Duration::days(1);

    let request = serde_json::json!({
        "check_out": next_check_out.to_string(),
        "room_class": "deluxe",
        "package_breakdowns": [
            {
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "300.00"
            },
            {
                "package_code": "DINNER",
                "revenue_category": "food_and_beverage",
                "amount": "80.00"
            }
        ],
        "daily_details": [
            {
                "service_date": created.check_in.to_string(),
                "room_class": "standard",
                "plan_code": "BB",
                "adult_count": 1,
                "child_count": 0
            },
            {
                "service_date": (created.check_in + Duration::days(1)).to_string(),
                "room_class": "deluxe",
                "plan_code": "HB",
                "adult_count": 2,
                "child_count": 1
            }
        ],
        "daily_revenue_allocations": [
            {
                "service_date": created.check_in.to_string(),
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "120.00"
            },
            {
                "service_date": (created.check_in + Duration::days(1)).to_string(),
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "180.00"
            },
            {
                "service_date": (created.check_in + Duration::days(1)).to_string(),
                "package_code": "DINNER",
                "revenue_category": "food_and_beverage",
                "amount": "80.00"
            }
        ]
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let modified: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(modified.check_out, next_check_out);
    assert_eq!(modified.room_class, "deluxe");
    assert_eq!(modified.package_breakdowns.len(), 2);
    assert_eq!(modified.daily_details.len(), 2);
    assert_eq!(modified.daily_revenue_allocations.len(), 3);
    assert!(modified.daily_details.iter().any(|detail| {
        detail.service_date == created.check_in + Duration::days(1)
            && detail.room_class == "deluxe"
            && detail.plan_code.as_deref() == Some("HB")
            && detail.adult_count == 2
            && detail.child_count == 1
    }));

    let mut tx = app.db.begin_tx().await;

    let breakdowns =
        SqliteReservationPackageBreakdownRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();
    let details =
        SqliteReservationDailyStayDetailRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();
    let allocations = SqliteReservationDailyRevenueAllocationRepository::list_by_reservation_id(
        &mut tx, created.id,
    )
    .await
    .unwrap();
    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();

    let modify_event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Modify
        })
        .unwrap();
    let changed_fields: Vec<ChangedField> =
        serde_json::from_str(&modify_event.changed_fields_json).unwrap();

    let _ = tx.rollback().await;

    assert_eq!(breakdowns.len(), 2);
    assert_eq!(details.len(), 2);
    assert_eq!(allocations.len(), 3);
    assert!(allocations.iter().any(|allocation| {
        allocation.service_date == created.check_in + Duration::days(1)
            && allocation.package_code == "DINNER"
            && allocation.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && allocation.amount == rust_decimal::Decimal::new(8000, 2)
    }));
    assert!(changed_fields
        .iter()
        .any(|field| field.field_name == "package_breakdowns"));
    assert!(changed_fields
        .iter()
        .any(|field| field.field_name == "daily_details"));
    assert!(changed_fields
        .iter()
        .any(|field| field.field_name == "daily_revenue_allocations"));
    assert!(changed_fields.iter().any(|field| {
        field.field_name == "package_breakdowns.DINNER.food_and_beverage"
            && field.before_value.is_none()
            && field.after_value.as_deref() == Some("80.00")
    }));
    assert!(changed_fields.iter().any(|field| {
        field.field_name
            == format!(
                "daily_details.{}.adult_count",
                created.check_in + Duration::days(1)
            )
            && field.before_value.is_none()
            && field.after_value.as_deref() == Some("2")
    }));
    assert!(changed_fields.iter().any(|field| {
        field.field_name
            == format!(
                "daily_revenue_allocations.{}.DINNER.food_and_beverage.amount",
                created.check_in + Duration::days(1)
            )
            && field.before_value.is_none()
            && field.after_value.as_deref() == Some("80.00")
    }));
}

#[tokio::test]
async fn should_reject_modified_daily_details_outside_reservation_range() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let request = serde_json::json!({
        "daily_details": [
            {
                "service_date": (created.check_out + Duration::days(1)).to_string(),
                "room_class": "standard",
                "adult_count": 1,
                "child_count": 0
            }
        ]
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_reject_modified_daily_revenue_allocations_outside_reservation_range() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let request = serde_json::json!({
        "daily_revenue_allocations": [
            {
                "service_date": created.check_out.to_string(),
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "100.00"
            }
        ]
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_preserve_daily_details_when_modifying_package_breakdowns_only() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let create_request = serde_json::json!({
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(2)).to_string(),
        "room_class": "standard",
        "daily_details": [
            {
                "service_date": today.to_string(),
                "room_class": "standard",
                "plan_code": "BB",
                "adult_count": 1,
                "child_count": 0
            },
            {
                "service_date": (today + Duration::days(1)).to_string(),
                "room_class": "deluxe",
                "plan_code": "HB",
                "adult_count": 2,
                "child_count": 1
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let create_response = post_json(&app.app, "/reservations", &create_request).await;

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    let modify_request = serde_json::json!({
        "package_breakdowns": [
            {
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "300.00"
            }
        ]
    });

    let modify_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &modify_request,
    )
    .await;

    assert_eq!(modify_response.status(), StatusCode::OK);

    let modified: ReservationResponse =
        serde_json::from_value(response_json(modify_response).await).unwrap();

    assert!(modified.daily_details.iter().any(|detail| {
        detail.service_date == today + Duration::days(1)
            && detail.room_class == "deluxe"
            && detail.plan_code.as_deref() == Some("HB")
            && detail.adult_count == 2
            && detail.child_count == 1
    }));
    assert_eq!(modified.daily_revenue_allocations.len(), 2);
    assert!(modified
        .daily_revenue_allocations
        .iter()
        .all(|allocation| allocation.amount == rust_decimal::Decimal::new(15000, 2)));
}

#[tokio::test]
async fn should_modify_reservation_participants() {
    let app = spawn_app().await;

    let original_guest = create_guest(&app.app).await;
    let new_primary_guest = create_guest(&app.app).await;
    let accompany_guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(original_guest.id).build();

    let request = ReservationBuilder::new()
        .with_participant(participant)
        .build();
    let create_response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    let modify_request = serde_json::json!({
        "participants": [
            {
                "guest_id": new_primary_guest.id.to_string(),
                "relation_type": "primary"
            },
            {
                "guest_id": accompany_guest.id.to_string(),
                "relation_type": "accompany"
            }
        ]
    });

    let modify_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &modify_request,
    )
    .await;

    assert_eq!(modify_response.status(), StatusCode::OK);

    let modified: ReservationResponse =
        serde_json::from_value(response_json(modify_response).await).unwrap();

    assert_eq!(modified.participants.len(), 2);
    assert!(modified.participants.iter().any(|participant| {
        participant.guest_id == new_primary_guest.id
            && participant.relation_type == ReservationGuestRelationType::Primary
    }));
    assert!(modified.participants.iter().any(|participant| {
        participant.guest_id == accompany_guest.id
            && participant.relation_type == ReservationGuestRelationType::Accompany
    }));

    let original_guest_response = get(
        &app.app,
        &format!("/guests/{}/reservations", original_guest.id),
    )
    .await;
    let new_primary_response = get(
        &app.app,
        &format!("/guests/{}/reservations", new_primary_guest.id),
    )
    .await;
    let accompany_response = get(
        &app.app,
        &format!("/guests/{}/reservations", accompany_guest.id),
    )
    .await;

    assert_eq!(original_guest_response.status(), StatusCode::OK);
    assert_eq!(new_primary_response.status(), StatusCode::OK);
    assert_eq!(accompany_response.status(), StatusCode::OK);

    let original_guest_reservations: Vec<ReservationResponse> =
        serde_json::from_value(response_json(original_guest_response).await).unwrap();
    let new_primary_reservations: Vec<ReservationResponse> =
        serde_json::from_value(response_json(new_primary_response).await).unwrap();
    let accompany_reservations: Vec<ReservationResponse> =
        serde_json::from_value(response_json(accompany_response).await).unwrap();

    assert!(original_guest_reservations
        .iter()
        .all(|reservation| reservation.id != created.id));
    assert!(new_primary_reservations
        .iter()
        .any(|reservation| reservation.id == created.id));
    assert!(accompany_reservations
        .iter()
        .any(|reservation| reservation.id == created.id));

    let mut tx = app.db.begin_tx().await;

    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();
    let modify_event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Modify
        })
        .unwrap();
    let changed_fields: Vec<ChangedField> =
        serde_json::from_str(&modify_event.changed_fields_json).unwrap();

    let _ = tx.rollback().await;

    assert!(changed_fields
        .iter()
        .any(|field| field.field_name == "participants"));
    assert!(changed_fields.iter().any(|field| {
        field.field_name == format!("participants.{}", original_guest.id)
            && field.before_value.as_deref() == Some("primary")
            && field.after_value.is_none()
    }));
    assert!(changed_fields.iter().any(|field| {
        field.field_name == format!("participants.{}", new_primary_guest.id)
            && field.before_value.is_none()
            && field.after_value.as_deref() == Some("primary")
    }));
}

#[tokio::test]
async fn should_reject_participant_modification_without_single_primary() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;
    let guest = create_guest(&app.app).await;

    let request = serde_json::json!({
        "participants": [
            {
                "guest_id": guest.id.to_string(),
                "relation_type": "accompany"
            }
        ]
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_mark_reservation_no_show_and_reinstate() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let no_show_response = post_json(
        &app.app,
        &format!("/reservations/{}/no-show", created.id),
        &serde_json::json!({}),
    )
    .await;

    assert_eq!(no_show_response.status(), StatusCode::OK);

    let no_show: ReservationResponse =
        serde_json::from_value(response_json(no_show_response).await).unwrap();

    assert_eq!(no_show.reservation_status, ReservationStatus::NoShow);
    assert_eq!(no_show.stay_status, Some(StayStatus::NoShow));

    let reinstate_response = post_json(
        &app.app,
        &format!("/reservations/{}/reinstate", created.id),
        &serde_json::json!({}),
    )
    .await;

    assert_eq!(reinstate_response.status(), StatusCode::OK);

    let reinstated: ReservationResponse =
        serde_json::from_value(response_json(reinstate_response).await).unwrap();

    assert_eq!(reinstated.reservation_status, ReservationStatus::Confirmed);
    assert_eq!(reinstated.stay_status, Some(StayStatus::Confirmed));

    let mut tx = app.db.begin_tx().await;

    let transitions =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();
    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();

    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::NoShowMarked
            && transition.before_value == "confirmed"
            && transition.after_value == "no_show"
    }));
    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::ReservationReinstated
            && transition.before_value == "no_show"
            && transition.after_value == "confirmed"
    }));

    let no_show_event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id
                && event.operation_type == OperationType::NoShow
                && event.after_json.contains("no_show")
        })
        .unwrap();
    let changed_fields: Vec<ChangedField> =
        serde_json::from_str(&no_show_event.changed_fields_json).unwrap();

    assert!(changed_fields.iter().any(|field| {
        field.field_name == "reservation_status"
            && field.before_value.as_deref() == Some("confirmed")
            && field.after_value.as_deref() == Some("no_show")
    }));
}

#[tokio::test]
async fn should_reject_no_show_after_check_in() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;
    let room = crate::common::room::create_room(&app.app).await;

    let assign_response = post_json(
        &app.app,
        &format!("/reservations/{}/assign-room/{}", created.id, room.id),
        &serde_json::json!({}),
    )
    .await;

    assert_eq!(assign_response.status(), StatusCode::OK);

    let check_in_response = post_json(
        &app.app,
        &format!("/reservations/{}/check-in", created.id),
        &serde_json::json!({}),
    )
    .await;

    assert_eq!(check_in_response.status(), StatusCode::OK);

    let no_show_response = post_json(
        &app.app,
        &format!("/reservations/{}/no-show", created.id),
        &serde_json::json!({}),
    )
    .await;

    assert_eq!(no_show_response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_record_semantic_reservation_transitions_on_modify() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let next_check_out = created.check_out + Duration::days(1);

    let request = serde_json::json!({
        "check_out": next_check_out.to_string(),
        "room_class": "deluxe"
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let mut tx = app.db.begin_tx().await;

    let transitions =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();

    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::CheckOutChanged
            && transition.field_name == "check_out"
            && transition.before_value == created.check_out.to_string()
            && transition.after_value == next_check_out.to_string()
    }));

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::ReservationExtended
            && transition.field_name == "check_out"
            && transition.before_value == created.check_out.to_string()
            && transition.after_value == next_check_out.to_string()
    }));

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::RoomClassChanged
            && transition.field_name == "room_class"
            && transition.before_value == created.room_class
            && transition.after_value == "deluxe"
    }));
}

#[tokio::test]
async fn should_record_operation_change_events_for_modify_and_cancel() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let modify_request = serde_json::json!({
        "check_out": (created.check_out + Duration::days(1)).to_string(),
        "room_class": "deluxe"
    });

    let modify_response = patch_json(
        &app.app,
        &format!("/reservations/{}", created.id),
        &modify_request,
    )
    .await;

    assert_eq!(modify_response.status(), StatusCode::OK);

    let cancel_response = delete(&app.app, &format!("/reservations/{}", created.id)).await;

    assert_eq!(cancel_response.status(), StatusCode::OK);

    let mut tx = app.db.begin_tx().await;

    let events = SqliteOperationChangeEventRepository::list_all(&mut tx)
        .await
        .unwrap();

    let modify_event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Modify
        })
        .unwrap();
    let cancel_event = events
        .iter()
        .find(|event| {
            event.aggregate_id == created.id && event.operation_type == OperationType::Cancel
        })
        .unwrap();

    assert!(modify_event.before_json.is_some());
    assert!(modify_event.after_json.contains("deluxe"));
    let modify_changed_fields: Vec<ChangedField> =
        serde_json::from_str(&modify_event.changed_fields_json).unwrap();
    let room_class_change = modify_changed_fields
        .iter()
        .find(|field| field.field_name == "room_class")
        .unwrap();
    assert_eq!(
        room_class_change.before_value.as_deref(),
        Some(created.room_class.as_str())
    );
    assert_eq!(room_class_change.after_value.as_deref(), Some("deluxe"));
    assert!(cancel_event.before_json.is_some());
    let cancel_changed_fields: Vec<ChangedField> =
        serde_json::from_str(&cancel_event.changed_fields_json).unwrap();
    assert!(cancel_changed_fields
        .iter()
        .any(|field| field.field_name == "reservation_status"));

    let modify_activation = fetch_semantic_activation(&mut tx, modify_event.id)
        .await
        .unwrap()
        .unwrap();
    let cancel_activation = fetch_semantic_activation(&mut tx, cancel_event.id)
        .await
        .unwrap()
        .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(
        modify_activation.activation_key,
        SemanticActivationKey::InventoryRelevantChange
    );
    assert_eq!(
        cancel_activation.activation_key,
        SemanticActivationKey::ReservationCancelled
    );
}

#[tokio::test]
async fn should_resolve_reservation_trace() {
    let app = spawn_app().await;
    let created = create_reservation(&app.app).await;

    let trace_response = post_json(
        &app.app,
        &format!("/reservations/{}/traces", created.id),
        &serde_json::json!({
            "department_code": "hk",
            "body": "Prepare extra towels",
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(trace_response.status(), StatusCode::CREATED);

    let trace_body = response_json(trace_response).await;
    let trace_id: Uuid = serde_json::from_value(trace_body["id"].clone()).unwrap();

    let resolve_response = post_json(
        &app.app,
        &format!("/reservations/{}/traces/{}/resolve", created.id, trace_id),
        &serde_json::json!({
            "actor_id": "hk-1"
        }),
    )
    .await;

    assert_eq!(resolve_response.status(), StatusCode::NO_CONTENT);

    let detail_response = get(&app.app, &format!("/reservations/{}", created.id)).await;
    assert_eq!(detail_response.status(), StatusCode::OK);

    let detail: ReservationResponse =
        serde_json::from_value(response_json(detail_response).await).unwrap();

    let trace = detail
        .traces
        .iter()
        .find(|trace| trace.id == trace_id)
        .unwrap();

    assert!(trace.resolved_at.is_some());
    assert_eq!(trace.resolved_by.as_deref(), Some("hk-1"));

    assert!(detail
        .audit_logs
        .iter()
        .any(|log| log.action == "reservation.trace.resolve"));
}

#[tokio::test]
async fn should_delete_reservation_note() {
    let app = spawn_app().await;
    let created = create_reservation(&app.app).await;

    let note_response = post_json(
        &app.app,
        &format!("/reservations/{}/notes", created.id),
        &serde_json::json!({
            "kind": "global_memo",
            "body": "Prefers quiet room",
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(note_response.status(), StatusCode::CREATED);

    let note_body = response_json(note_response).await;
    let note_id: Uuid = serde_json::from_value(note_body["id"].clone()).unwrap();

    let delete_response = delete_json(
        &app.app,
        &format!("/reservations/{}/notes/{}", created.id, note_id),
        &serde_json::json!({
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let detail_response = get(&app.app, &format!("/reservations/{}", created.id)).await;
    assert_eq!(detail_response.status(), StatusCode::OK);

    let detail: ReservationResponse =
        serde_json::from_value(response_json(detail_response).await).unwrap();

    assert!(detail.notes.iter().all(|note| note.id != note_id));

    assert!(detail
        .audit_logs
        .iter()
        .any(|log| log.action == "reservation.memo.delete"));
}

#[tokio::test]
async fn should_delete_reservation_trace() {
    let app = spawn_app().await;
    let created = create_reservation(&app.app).await;

    let trace_response = post_json(
        &app.app,
        &format!("/reservations/{}/traces", created.id),
        &serde_json::json!({
            "department_code": "hk",
            "body": "Prepare extra towels",
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(trace_response.status(), StatusCode::CREATED);

    let trace_body = response_json(trace_response).await;
    let trace_id: Uuid = serde_json::from_value(trace_body["id"].clone()).unwrap();

    let delete_response = delete_json(
        &app.app,
        &format!("/reservations/{}/traces/{}", created.id, trace_id),
        &serde_json::json!({
            "actor_id": "front-1"
        }),
    )
    .await;

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let detail_response = get(&app.app, &format!("/reservations/{}", created.id)).await;
    assert_eq!(detail_response.status(), StatusCode::OK);

    let detail: ReservationResponse =
        serde_json::from_value(response_json(detail_response).await).unwrap();

    assert!(detail.traces.iter().all(|trace| trace.id != trace_id));

    assert!(detail
        .audit_logs
        .iter()
        .any(|log| log.action == "reservation.trace.delete"));
}
