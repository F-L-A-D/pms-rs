use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use tower::ServiceExt;

use serde_json::Value;

use crate::helpers::{
    app::test_app,

    guest::{
        create_guest_with,
    },

    builders::guest_builder::
        GuestBuilder,
};

#[tokio::test]
async fn should_search_guest_without_noise() {

    let app =
        test_app().await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Sato",
                "Takashi",
            )
            .with_email(
                "takashi@example.com",
            )
            .with_phone(
                "09011111111",
            )
    )
    .await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Sato",
                "Aoi",
            )
            .with_email(
                "aoi@example.com",
            )
            .with_phone(
                "09022222222",
            )
    )
    .await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Suzuki",
                "Jiro",
            )
    )
    .await;

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        "/guests?query=Sato%20Takashi"
                    )
                    .body(
                        Body::empty()
                    )
                    .unwrap(),
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

    let guests: Vec<Value> =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    assert_eq!(
        guests.len(),
        1,
    );

    assert_eq!(
        guests[0]["last_name"],
        "Sato",
    );

    assert_eq!(
        guests[0]["first_name"],
        "Takashi",
    );
}

#[tokio::test]
async fn should_update_guest() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Sato",
                    "Takashi",
                )
                .with_email(
                    "takashi@example.com",
                )
                .with_phone(
                    "09011111111",
                )
        )
        .await;

    let update_payload =
        serde_json::json!({

            "last_name":
                "Sato",

            "first_name":
                "Updated",

            "phone":
                "09099999999",

            "email":
                "updated@example.com",

            "nationality":
                "JP",

            "birth_date":
                null,

            "gender":
                null,

            "membership_code":
                "UPDATED-001",

            "marketing_opt_in":
                true
        });

    let response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(
                        &format!(
                            "/guests/{}",
                            guest_id,
                        )
                    )
                    .header(
                        "content-type",
                        "application/json",
                    )
                    .body(
                        Body::from(
                            update_payload.to_string()
                        )
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let get_response =
        app.app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(
                        &format!(
                            "/guests/{}",
                            guest_id,
                        )
                    )
                    .body(
                        Body::empty()
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

    let body =
        axum::body::to_bytes(
            get_response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let guest: Value =
        serde_json::from_slice(
            &body
        )
        .unwrap();

    assert_eq!(
        guest["first_name"],
        "Updated",
    );

    assert_eq!(
        guest["email"],
        "updated@example.com",
    );

    assert_eq!(
        guest["membership_code"],
        "UPDATED-001",
    );

    assert_eq!(
        guest["marketing_opt_in"],
        true,
    );
}