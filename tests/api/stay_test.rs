use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use serde_json::json;

use tower::ServiceExt;

use crate::helpers::{
    app::test_app,
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_assign_room() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app).await;

    create_reservation(
        &app,
        "reservation-001",
        guest_id,
    )
    .await;

    let room_payload =
        json!({
            "id": "room-101",
            "room_class": "STD"
        });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/rooms")
                .header(
                    "content-type",
                    "application/json",
                )
                .body(
                    Body::from(
                        room_payload.to_string()
                    )
                )
                .unwrap()
        )
        .await
        .unwrap();

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        "/stays/reservation-001/assign-room/room-101"
                    )
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );
}

#[tokio::test]
async fn should_check_in() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app).await;

    create_reservation(
        &app,
        "reservation-001",
        guest_id,
    )
    .await;

    let room_payload =
        json!({
            "id": "room-101",
            "room_class": "STD"
        });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/rooms")
                .header(
                    "content-type",
                    "application/json",
                )
                .body(
                    Body::from(
                        room_payload.to_string()
                    )
                )
                .unwrap()
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/stays/reservation-001/assign-room/room-101"
                )
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(
                        "/stays/reservation-001/check-in"
                    )
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );
}