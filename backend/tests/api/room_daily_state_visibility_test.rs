use axum::http::StatusCode;

use serde_json::json;

use crate::common::{
    app::spawn_app,
    client::{get, post_json, response_json},
    room::create_room,
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