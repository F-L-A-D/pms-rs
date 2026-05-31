use axum::http::StatusCode;

use chrono::Duration;

use pms_rs::api::dto::response::reservation::ReservationResponse;

use crate::common::{
    app::spawn_app,
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    business_date::{
        current_open_business_date, get_current_business_date, get_night_audit_worklist,
        post_night_audit_room_charges, start_night_audit,
    },
    client::{get, post, post_json, response_json},
    guest::create_guest,
    housekeeping::inspect_room_for_date,
    room::create_room,
    stay::{assign_room, check_in, check_out},
};

#[tokio::test]
async fn should_complete_business_date_stay_and_night_audit_workflow() {
    let app = spawn_app().await;

    let current = get_current_business_date(&app.app).await;

    assert_eq!(current["business_date"], "2026-06-01");
    assert_eq!(current["status"], "open");

    let business_date = current_open_business_date(&app.app).await;
    let reservation = create_room_charge_reservation(&app.app, "NA-WORKFLOW-001", 1).await;

    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;

    inspect_room_for_date(&app.app, old_room.id, business_date).await;
    inspect_room_for_date(&app.app, new_room.id, business_date).await;

    let checked_in = check_in(&app.app, reservation.id).await;
    assert_eq!(checked_in.status, "checked_in");

    let response = post_json(
        &app.app,
        &format!("/reservations/{}/room-move/{}", reservation.id, new_room.id),
        &serde_json::json!({ "effective_date": business_date.to_string() }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    assert_eq!(body["status"], "room_moved");

    start_night_audit(&app.app).await;

    let worklist = get_night_audit_worklist(&app.app).await;
    assert_eq!(
        worklist["room_charge_candidates"].as_array().unwrap().len(),
        1
    );
    assert_eq!(worklist["room_charge_candidates"][0]["amount"], "100.00");

    let posted = post_night_audit_room_charges(&app.app).await;
    assert_eq!(posted["posted_room_charges"].as_array().unwrap().len(), 1);

    let posted_again = post_night_audit_room_charges(&app.app).await;
    assert_eq!(
        posted_again["posted_room_charges"]
            .as_array()
            .unwrap()
            .len(),
        0
    );

    let folio_id = posted["posted_room_charges"][0]["folio_id"]
        .as_str()
        .unwrap();
    let response = get(&app.app, &format!("/folios/{}", folio_id)).await;
    assert_eq!(response.status(), StatusCode::OK);
    let folio = response_json(response).await;
    assert!(folio["entries"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| { entry["entry_type"] == "room_charge" && entry["amount"] == "100.00" }));

    let response = post_json(
        &app.app,
        "/business-date/night-audit/finalize",
        &serde_json::json!({ "reason": "test finalize" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let current = get_current_business_date(&app.app).await;

    assert_eq!(current["business_date"], "2026-06-02");
    assert_eq!(current["status"], "open");

    let checked_out = check_out(&app.app, reservation.id).await;
    assert_eq!(checked_out.status, "checked_out");

    let response = get(
        &app.app,
        &format!(
            "/rooms/{}?service_date={}",
            new_room.id,
            (business_date + Duration::days(1))
        ),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    assert_eq!(body["daily_state"]["occupancy_status"], "vacant");
    assert_eq!(body["daily_state"]["housekeeping_status"], "dirty");
}

#[tokio::test]
async fn should_reject_night_audit_finalize_with_unresolved_arrivals() {
    let app = spawn_app().await;

    create_current_business_date_reservation(&app.app).await;
    start_night_audit(&app.app).await;

    let response = post_json(
        &app.app,
        "/business-date/night-audit/finalize",
        &serde_json::json!({ "reason": "test finalize" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_resolve_unresolved_arrival_as_no_show_during_night_audit() {
    let app = spawn_app().await;

    let reservation = create_current_business_date_reservation(&app.app).await;
    start_night_audit(&app.app).await;

    let worklist = get_night_audit_worklist(&app.app).await;
    assert_eq!(worklist["unresolved_arrivals"].as_array().unwrap().len(), 1);

    let response = post(
        &app.app,
        &format!(
            "/business-date/night-audit/arrivals/{}/no-show",
            reservation.id
        ),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = response_json(response).await;
    assert_eq!(body["reservation_status"], "no_show");
    assert_eq!(body["stay_status"], "no_show");

    let worklist = get_night_audit_worklist(&app.app).await;
    assert!(worklist["unresolved_arrivals"]
        .as_array()
        .unwrap()
        .is_empty());

    let response = post_json(
        &app.app,
        "/business-date/night-audit/finalize",
        &serde_json::json!({ "reason": "test finalize" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_reject_future_arrival_no_show_during_night_audit() {
    let app = spawn_app().await;
    let business_date = current_open_business_date(&app.app).await;
    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(business_date + Duration::days(1))
        .with_check_out(business_date + Duration::days(2))
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    start_night_audit(&app.app).await;

    let response = post(
        &app.app,
        &format!(
            "/business-date/night-audit/arrivals/{}/no-show",
            reservation.id
        ),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_reject_night_audit_finalize_with_unposted_room_charges() {
    let app = spawn_app().await;

    let business_date = current_open_business_date(&app.app).await;
    let reservation = create_room_charge_reservation(&app.app, "NA-UNPOSTED-001", 1).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    inspect_room_for_date(&app.app, room.id, business_date).await;
    check_in(&app.app, reservation.id).await;
    start_night_audit(&app.app).await;

    let response = post_json(
        &app.app,
        "/business-date/night-audit/finalize",
        &serde_json::json!({ "reason": "test finalize" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_defer_unresolved_arrival_during_night_audit() {
    let app = spawn_app().await;

    let business_date = current_open_business_date(&app.app).await;
    let reservation = create_room_charge_reservation(&app.app, "NA-DEFER-001", 2).await;

    start_night_audit(&app.app).await;

    let worklist = get_night_audit_worklist(&app.app).await;
    assert_eq!(worklist["unresolved_arrivals"].as_array().unwrap().len(), 1);

    let response = post_json(
        &app.app,
        &format!("/reservations/{}/defer-arrival", reservation.id),
        &serde_json::json!({ "post_room_charge": true }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let deferred: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();
    assert_eq!(deferred.check_in, business_date + Duration::days(1));
    assert_eq!(deferred.check_out, business_date + Duration::days(2));

    let worklist = get_night_audit_worklist(&app.app).await;
    assert!(worklist["unresolved_arrivals"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(worklist["room_charge_candidates"]
        .as_array()
        .unwrap()
        .is_empty());

    let response = post_json(
        &app.app,
        "/business-date/night-audit/finalize",
        &serde_json::json!({ "reason": "test finalize" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_reject_housekeeping_when_service_date_does_not_match_current_business_date() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/dirty", room.id),
        &serde_json::json!({ "service_date": "2026-06-02" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_reject_check_in_when_reservation_check_in_does_not_match_current_business_date() {
    let app = spawn_app().await;
    let business_date = current_open_business_date(&app.app).await;
    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(business_date + Duration::days(1))
        .with_check_out(business_date + Duration::days(2))
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let response = post(
        &app.app,
        &format!("/reservations/{}/check-in", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_reject_check_out_when_reservation_check_out_does_not_match_current_business_date() {
    let app = spawn_app().await;
    let reservation = create_current_business_date_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    inspect_room_for_date(&app.app, room.id, reservation.check_in).await;
    check_in(&app.app, reservation.id).await;

    let response = post(
        &app.app,
        &format!("/reservations/{}/check-out", reservation.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn should_reject_room_move_when_effective_date_does_not_match_current_business_date() {
    let app = spawn_app().await;
    let business_date = current_open_business_date(&app.app).await;
    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(business_date)
        .with_check_out(business_date + Duration::days(2))
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();
    let old_room = create_room(&app.app).await;
    let new_room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, old_room.id).await;
    inspect_room_for_date(&app.app, old_room.id, business_date).await;
    check_in(&app.app, reservation.id).await;

    let response = post_json(
        &app.app,
        &format!("/reservations/{}/room-move/{}", reservation.id, new_room.id),
        &serde_json::json!({ "effective_date": (business_date + Duration::days(1)).to_string() }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

async fn create_current_business_date_reservation(app: &axum::Router) -> ReservationResponse {
    let guest = create_guest(app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let request = ReservationBuilder::new()
        .with_participant(participant)
        .build();

    let response = post_json(app, "/reservations", &request).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    serde_json::from_value(response_json(response).await).unwrap()
}

async fn create_room_charge_reservation(
    app: &axum::Router,
    external_id: &str,
    nights: i64,
) -> ReservationResponse {
    let business_date = current_open_business_date(app).await;
    let guest = create_guest(app).await;

    let response = post_json(
        app,
        "/reservations",
        &serde_json::json!({
            "external_id": external_id,
            "check_in": business_date.to_string(),
            "check_out": (business_date + Duration::days(nights)).to_string(),
            "room_class": "standard",
            "booking_channel": "direct",
            "package_breakdowns": [
                {
                    "package_code": "ROOM",
                    "revenue_category": "room",
                    "amount": "100.00"
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

    assert_eq!(response.status(), StatusCode::CREATED);

    serde_json::from_value(response_json(response).await).unwrap()
}
