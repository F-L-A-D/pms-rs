use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use crate::helpers::{
    app::test_app,
    billing::{open_folio, post_payment, post_room_charge},
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_open_folio() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(
            &app.app
        )
        .await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    assert_ne!(
        folio_id,
        uuid::Uuid::nil(),
    );
}

#[tokio::test]
async fn should_post_room_charge() {
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
}

#[tokio::test]
async fn should_post_payment() {
    let app = test_app().await;

    let guest_id = create_guest(&app.app).await;

    let reservation_id = create_reservation(&app.app, "reservation-001", guest_id).await;

    let folio_id =
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_payment(&app.app, folio_id, 12000).await;
}

#[tokio::test]
async fn should_get_balance() {
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

    post_payment(&app.app, folio_id, 5000).await;

    let response = app
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    &format!(
                        "/folios/{}/balance",
                        folio_id
                    )
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK,);
}
