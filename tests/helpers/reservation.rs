use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
    Router,
};

use tower::ServiceExt;

use uuid::Uuid;

use serde_json::Value;

use crate::helpers::builders::
    reservation_request_builder::
    ReservationRequestBuilder;

pub async fn create_reservation(
    app: &Router,
    external_id: &str,
    guest_id: Uuid,
) -> Uuid {

    let builder =
        ReservationRequestBuilder::new()
            .with_external_id(
                external_id,
            )
            .with_primary_guest(
                guest_id,
            );

    create_reservation_with(
        app,
        builder,
    )
    .await
}

pub async fn create_reservation_with(
    app: &Router,
    builder: ReservationRequestBuilder,
) -> Uuid {

    let payload =
        builder.build();

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

    let body =
        axum::body::to_bytes(
            response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let json: Value =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    Uuid::parse_str(
        json["id"]
            .as_str()
            .unwrap()
    )
    .unwrap()
}

pub async fn modify_reservation(
    app: &Router,
    reservation_id: Uuid,
    payload: Value,
) {

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(
                        &format!(
                            "/reservations/{}",
                            reservation_id,
                        )
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