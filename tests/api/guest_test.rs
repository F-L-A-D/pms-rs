use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use tower::ServiceExt;

use serde_json::{
    json,
    Value,
};

use crate::helpers::app::test_app;

#[tokio::test]
async fn should_search_guest_without_noise() {

    let app =
        test_app().await;

    let guests = vec![

        json!({
            "last_name": "Sato",
            "first_name": "Takashi",
            "phone": "09011111111",
            "email": "takashi@example.com",
            "nationality": null,
            "birth_date": null,
            "gender": null,
            "membership_code": "TAKASHI-001",
            "marketing_opt_in": false
        }),

        json!({
            "last_name": "Sato",
            "first_name": "Aoi",
            "phone": "09022222222",
            "email": "aoi@example.com",
            "nationality": null,
            "birth_date": null,
            "gender": null,
            "membership_code": "AOI-001",
            "marketing_opt_in": false
        }),
    ];

    for guest in guests {

        app.app.clone()
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
                            guest.to_string()
                        )
                    )
                    .unwrap(),
            )
            .await
            .unwrap();
    }

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
        serde_json::from_slice(&body)
            .unwrap();

    assert_eq!(
        guests.len(),
        1,
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

    let create_payload =
        json!({
            "last_name": "Sato",
            "first_name": "Takashi",
            "phone": "09011111111",
            "email": "takashi@example.com",
            "nationality": null,
            "birth_date": null,
            "gender": null,
            "membership_code": "TAKASHI-001",
            "marketing_opt_in": false
        });

    let create_response =
        app.app.clone()
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
                            create_payload.to_string()
                        )
                    )
                    .unwrap(),
            )
            .await
            .unwrap();

    let create_body =
        axum::body::to_bytes(
            create_response.into_body(),
            usize::MAX,
        )
        .await
        .unwrap();

    let created_guest: Value =
        serde_json::from_slice(
            &create_body
        )
        .unwrap();

    let guest_id =
        created_guest["id"]
            .as_str()
            .unwrap();

    let update_payload =
        json!({
            "last_name": "Sato",
            "first_name": "Updated",
            "phone": "09099999999",
            "email": "updated@example.com",
            "nationality": "JP",
            "birth_date": null,
            "gender": null,
            "membership_code": "UPDATED-001",
            "marketing_opt_in": true
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