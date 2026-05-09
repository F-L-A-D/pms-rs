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
    guest_builder::
    GuestBuilder;

pub async fn create_guest(
    app: &Router,
) -> Uuid {

    create_guest_with(
        app,
        GuestBuilder::new(),
    )
    .await
}

pub async fn create_guest_with(
    app: &Router,
    builder: GuestBuilder,
) -> Uuid {

    let payload =
        builder.build();

    let response =
        app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/guests")
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