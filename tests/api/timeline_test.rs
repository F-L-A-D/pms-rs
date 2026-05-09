use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::Value;

use tower::ServiceExt;

use crate::helpers::{app::test_app, guest::create_guest, reservation::create_reservation};

#[tokio::test]
async fn should_record_guest_timeline() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/guests/{guest_id}/timeline"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let arr = json.as_array().unwrap();

    assert_eq!(arr.len(), 1,);

    assert_eq!(arr[0]["event_type"], "ReservationCreated",);

    assert_eq!(arr[0]["reference_id"], reservation_id.to_string(),);
}

#[tokio::test]
async fn should_return_empty_timeline() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/guests/{guest_id}/timeline"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: Value = serde_json::from_slice(&body).unwrap();

    let arr = json.as_array().unwrap();

    assert_eq!(arr.len(), 0,);
}
