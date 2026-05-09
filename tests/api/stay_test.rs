use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::json;

use tower::ServiceExt;

use crate::helpers::{app::test_app, guest::create_guest, reservation::create_reservation};

#[tokio::test]
async fn should_assign_room() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let room_payload = json!({
        "id": "room-101",
        "room_class": "STD"
    });

    app.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/rooms")
                .header("content-type", "application/json")
                .body(Body::from(room_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/stays/reservation-001/assign-room/room-101"
                        .replace("reservation-001", &reservation_id.to_string()),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);
}

#[tokio::test]
async fn should_check_in() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let room_payload = json!({
        "id": "room-101",
        "room_class": "STD"
    });

    app.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/rooms")
                .header("content-type", "application/json")
                .body(Body::from(room_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    app.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/stays/reservation-001/assign-room/room-101"
                        .replace("reservation-001", &reservation_id.to_string()),
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/stays/reservation-001/check-in".replace("reservation-001", &reservation_id.to_string()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);
}
