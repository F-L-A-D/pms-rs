use axum::http::StatusCode;

use pms_rs::api::dto::response::guest::GuestResponse;

#[allow(unused_imports)]
use crate::common::{
    app::spawn_app,
    builders::GuestBuilder,
    client::{get, patch_json, post_json, response_json},
    guest::{create_guest, update_guest},
};

#[tokio::test]
async fn should_create_guest() {
    let app = spawn_app().await;

    let request = GuestBuilder::new().build();

    let response = post_json(&app.app, "/guests", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);
}

#[tokio::test]
async fn should_roundtrip_guest() {
    let app = spawn_app().await;

    let created = create_guest(&app.app).await;

    let response = get(&app.app, &format!("/guests/{}", created.id,)).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let retrieved: GuestResponse = serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(retrieved.id, created.id,);

    assert_eq!(retrieved.first_name, created.first_name,);

    assert_eq!(retrieved.last_name, created.last_name,);
}

#[tokio::test]
async fn should_update_guest() {
    let app = spawn_app().await;

    let created = create_guest(&app.app).await;

    let updated = update_guest(&app.app, created.id).await;

    let response = get(&app.app, &format!("/guests/{}", created.id,)).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let retrieved: GuestResponse = serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(retrieved.id, created.id,);

    assert_eq!(retrieved.first_name, updated.first_name,);

    assert_eq!(retrieved.last_name, updated.last_name,);

    assert_eq!(retrieved.email, updated.email,);

    assert_ne!(retrieved.first_name, created.first_name,);
}

#[tokio::test]
async fn should_create_and_list_guest_preferences() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let request = serde_json::json!({
        "preference_type": "pillow",
        "value": "firm pillow",
        "notes": "Prefers two pillows"
    });

    let create_response = post_json(
        &app.app,
        &format!("/guests/{}/preferences", guest.id),
        &request,
    )
    .await;

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let created = response_json(create_response).await;

    assert_eq!(created["guest_id"], guest.id.to_string());
    assert_eq!(created["preference_type"], "pillow");
    assert_eq!(created["value"], "firm pillow");

    let list_response = get(&app.app, &format!("/guests/{}/preferences", guest.id)).await;

    assert_eq!(list_response.status(), StatusCode::OK);

    let preferences = response_json(list_response).await;
    let preferences = preferences.as_array().unwrap();

    assert_eq!(preferences.len(), 1);
    assert_eq!(preferences[0]["preference_type"], "pillow");
    assert_eq!(preferences[0]["value"], "firm pillow");
}

#[tokio::test]
async fn should_reject_guest_preference_for_missing_guest() {
    let app = spawn_app().await;

    let request = serde_json::json!({
        "preference_type": "room",
        "value": "high floor"
    });

    let response = post_json(
        &app.app,
        &format!("/guests/{}/preferences", uuid::Uuid::new_v4()),
        &request,
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
