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
async fn should_create_reservation() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    let payload =
        json!({
            "id": "reservation-001",
            "check_in": "2026-05-10",
            "nights": 2,
            "room_class": "STD",
            "primary_guest_id": "guest-001"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/reservations")
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            payload.to_string()
                        )
                    )
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
async fn should_fail_when_guest_not_found() {

    let app =
        test_app().await;

    let payload =
        json!({
            "id": "reservation-001",
            "check_in": "2026-05-10",
            "nights": 2,
            "room_class": "STD",
            "primary_guest_id": "guest-999"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/reservations")
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            payload.to_string()
                        )
                    )
                    .unwrap()
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
    );
}

#[tokio::test]
async fn should_modify_reservation() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    let payload =
        json!({
            "check_in": "2026-05-15",
            "nights": 3,
            "room_class": "DLX"
        });

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(
                        "/reservations/reservation-001"
                    )
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            payload.to_string()
                        )
                    )
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
async fn should_cancel_reservation() {

    let app =
        test_app().await;

    create_guest(
        &app,
        "guest-001",
    )
    .await;

    create_reservation(
        &app,
        "reservation-001",
        "guest-001",
    )
    .await;

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(
                        "/reservations/reservation-001"
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