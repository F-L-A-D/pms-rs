use axum::http::StatusCode;

use pms_rs::api::dto::guest::GuestResponse;

#[allow(unused_imports)]
use crate::common::{
    app::spawn_app,

    builders::GuestBuilder,
    
    client::{
        get,
        patch_json,
        post_json,
        response_json,
    },
    
    guest::{
        create_guest,
        update_guest,
    },
};

#[tokio::test]
async fn should_create_guest() {

    let app = spawn_app().await;

    let request =
        GuestBuilder::new()
            .build();

    let response =
        post_json(
            &app.app,
            "/guests",
            &request,
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
    );
}

#[tokio::test]
async fn should_roundtrip_guest() {

    let app = spawn_app().await;

    let created =
        create_guest(&app.app)
            .await;

    let response =
        get(
            &app.app,
            &format!(
                "/guests/{}",
                created.id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let retrieved: GuestResponse =
        serde_json::from_value(
            response_json(response)
                .await,
        )
        .unwrap();

    assert_eq!(
        retrieved.id,
        created.id,
    );

    assert_eq!(
        retrieved.first_name,
        created.first_name,
    );

    assert_eq!(
        retrieved.last_name,
        created.last_name,
    );
}

#[tokio::test]
async fn should_update_guest() {

    let app = spawn_app().await;

    let created =
        create_guest(&app.app)
            .await;

    let updated =
        update_guest(
            &app.app,
            created.id,
        )
        .await;

    let response =
        get(
            &app.app,
            &format!(
                "/guests/{}",
                created.id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    let retrieved: GuestResponse =
        serde_json::from_value(
            response_json(response)
                .await,
        )
        .unwrap();

    assert_eq!(
        retrieved.id,
        created.id,
    );

    assert_eq!(
        retrieved.first_name,
        updated.first_name,
    );

    assert_eq!(
        retrieved.last_name,
        updated.last_name,
    );

    assert_eq!(
        retrieved.email,
        updated.email,
    );

    assert_ne!(
        retrieved.first_name,
        created.first_name,
    );
}