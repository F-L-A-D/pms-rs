use axum::http::StatusCode;

use serde_json::json;

use crate::common::{
    app::spawn_app,
    client::{get, post_json, response_json},
    room::create_room,
    reservation::{create_reservation, cancel_reservation, mark_no_show},
    stay::assign_room,
};

#[tokio::test]
async fn should_list_rooms_with_null_daily_state_when_service_date_has_no_state() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;

    let response = get(&app.app, "/rooms?service_date=2026-05-17").await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(room_json["room_no"], room.room_no);
    assert_eq!(room_json["room_class"], room.room_class);
    assert!(room_json["daily_state"].is_null());
}

#[tokio::test]
async fn should_list_rooms_with_daily_state_after_room_is_marked_out_of_order() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = "2026-05-17";

    let response = post_json(
        &app.app,
        &format!("/rooms/{}/out-of-order", room.id),
        &json!({ "service_date": service_date }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = get(
        &app.app,
        &format!("/rooms?service_date={}", service_date),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(
        room_json["daily_state"]["room_id"],
        room.id.to_string(),
    );
    assert_eq!(
        room_json["daily_state"]["service_date"],
        service_date,
    );
    assert_eq!(
        room_json["daily_state"]["occupancy_status"],
        "out_of_order",
    );
}

#[tokio::test]
async fn should_list_rooms_with_daily_state_after_housekeeping_transition() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = "2026-05-17";

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/dirty", room.id),
        &json!({ "service_date": service_date }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/start-cleaning", room.id),
        &json!({ "service_date": service_date }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = get(
        &app.app,
        &format!("/rooms?service_date={}", service_date),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(
        room_json["daily_state"]["occupancy_status"],
        "vacant",
    );
    assert_eq!(
        room_json["daily_state"]["housekeeping_status"],
        "cleaning",
    );
}

#[tokio::test]
async fn should_reject_invalid_room_list_service_date() {
    let app = spawn_app().await;

    let response = get(&app.app, "/rooms?service_date=2026/05/17").await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_get_room_with_daily_state_for_service_date() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = "2026-05-17";

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/dirty", room.id),
        &json!({ "service_date": service_date }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let response = get(
        &app.app,
        &format!("/rooms/{}?service_date={}", room.id, service_date),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;

    assert_eq!(body_json["id"], room.id.to_string());
    assert_eq!(body_json["room_no"], room.room_no);
    assert_eq!(
        body_json["daily_state"]["room_id"],
        room.id.to_string(),
    );
    assert_eq!(
        body_json["daily_state"]["service_date"],
        service_date,
    );
    assert_eq!(
        body_json["daily_state"]["housekeeping_status"],
        "dirty",
    );
}

#[tokio::test]
async fn should_get_room_with_null_daily_state_when_no_state_exists() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = "2026-05-17";

    let response = get(
        &app.app,
        &format!("/rooms/{}?service_date={}", room.id, service_date),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;

    assert_eq!(body_json["id"], room.id.to_string());
    assert!(body_json["daily_state"].is_null());
}

#[tokio::test]
async fn should_reject_invalid_get_room_service_date() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;

    let response = get(
        &app.app,
        &format!("/rooms/{}?service_date=2026/05/17", room.id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_show_assigned_room_as_assigned_but_not_occupied_before_check_in() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;

    let response = get(
        &app.app,
        &format!("/rooms?service_date={}", reservation.check_in),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(
        room_json["assignment"]["assignment_status"],
        "assigned",
    );
    assert_eq!(
        room_json["assignment"]["reservation_id"],
        reservation.id.to_string(),
    );
    assert_eq!(
        room_json["assignment"]["stay_status"],
        "confirmed",
    );

    assert!(room_json["daily_state"].is_null());
}

#[tokio::test]
async fn should_show_no_show_room_assignment_as_warning() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    mark_no_show(&app.app, reservation.id).await;

    let response = get(
        &app.app,
        &format!("/rooms?service_date={}", reservation.check_in),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(
        room_json["assignment"]["assignment_status"],
        "unassigned",
    );
    assert_eq!(
        room_json["assignment"]["reservation_id"],
        reservation.id.to_string(),
    );
    assert_eq!(
        room_json["assignment"]["warning"],
        "no_show_reservation_still_linked_to_room",
    );
}

#[tokio::test]
async fn should_ignore_cancelled_room_assignment() {
    let app = spawn_app().await;

    let reservation = create_reservation(&app.app).await;
    let room = create_room(&app.app).await;

    assign_room(&app.app, reservation.id, room.id).await;
    cancel_reservation(&app.app, reservation.id).await;

    let response = get(
        &app.app,
        &format!("/rooms?service_date={}", reservation.check_in),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    let rooms = body_json["rooms"].as_array().unwrap();

    let room_json = rooms
        .iter()
        .find(|item| item["id"] == room.id.to_string())
        .unwrap();

    assert_eq!(
        room_json["assignment"]["assignment_status"],
        "unassigned",
    );
    assert!(room_json["assignment"]["reservation_id"].is_null());
    assert!(room_json["assignment"]["warning"].is_null());
}