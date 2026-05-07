use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use serde_json::json;

use tower::ServiceExt;

use crate::helpers::app::test_app;

#[tokio::test]
async fn should_mark_dirty() {

    let app =
        test_app().await;

    let payload =
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
                        payload.to_string()
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
                        "/housekeeping/room-101/dirty"
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
async fn should_finish_cleaning_flow() {

    let app =
        test_app().await;

    let payload =
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
                        payload.to_string()
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
                    "/housekeeping/room-101/dirty"
                )
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/housekeeping/room-101/start-cleaning"
                )
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/housekeeping/room-101/finish-cleaning"
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
                        "/housekeeping/room-101/inspect"
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