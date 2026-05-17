use axum::http::StatusCode;

use serde_json::json;

use uuid::Uuid;

use crate::common::{
    app::spawn_app,
    client::{post_json, response_json},
    room::create_room,
};

#[tokio::test]
async fn should_run_housekeeping_lifecycle_for_room_daily_state() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let body = json!({ "service_date": "2026-05-17" });

    let response = post_json(&app.app, &format!("/housekeeping/{}/dirty", room.id), &body).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body_json = response_json(response).await;
    assert_eq!(body_json["room_id"], room.id.to_string());
    assert_eq!(body_json["service_date"], "2026-05-17");
    assert_eq!(body_json["occupancy_status"], "vacant");
    assert_eq!(body_json["housekeeping_status"], "dirty");

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/start-cleaning", room.id),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body_json = response_json(response).await;
    assert_eq!(body_json["housekeeping_status"], "cleaning");

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/finish-cleaning", room.id),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body_json = response_json(response).await;
    assert_eq!(body_json["housekeeping_status"], "cleaned");

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/inspect", room.id),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let body_json = response_json(response).await;
    assert_eq!(body_json["housekeeping_status"], "inspected");
}

#[tokio::test]
async fn should_reject_housekeeping_for_missing_room() {
    let app = spawn_app().await;
    let room_id = Uuid::new_v4();
    let body = json!({ "service_date": "2026-05-17" });

    let response = post_json(&app.app, &format!("/housekeeping/{}/dirty", room_id), &body).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn should_reject_invalid_housekeeping_transition() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let body = json!({ "service_date": "2026-05-17" });

    let response = post_json(
        &app.app,
        &format!("/housekeeping/{}/finish-cleaning", room.id),
        &body,
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn should_reject_invalid_housekeeping_service_date() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let body = json!({ "service_date": "2026/05/17" });

    let response = post_json(&app.app, &format!("/housekeeping/{}/dirty", room.id), &body).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
