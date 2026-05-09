use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use serde_json::Value;

use tower::ServiceExt;

use crate::helpers::{
    app::test_app,
    billing::{open_folio, post_room_charge},
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_get_guest_summary() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_room_charge(&app.app, folio_id, 12000).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/guests/{guest_id}/summary"))
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

    assert_eq!(json["total_stays"], 1,);

    assert_eq!(json["total_nights"], 2,);

    assert_eq!(json["total_spending"], 12000,);

    assert_eq!(json["last_stay_at"], "2026-05-10",);
}

#[tokio::test]
async fn should_return_zero_metrics() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/guests/{guest_id}/summary"))
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

    assert_eq!(json["total_stays"], 0,);

    assert_eq!(json["total_nights"], 0,);

    assert_eq!(json["total_spending"], 0,);

    assert!(json["last_stay_at"].is_null());
}
