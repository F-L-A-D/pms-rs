use axum::{http::StatusCode, Router};

use pms_rs::api::dto::guest::{CreateGuestRequest, GuestResponse, UpdateGuestRequest};

#[allow(unused_imports)]
use super::{
    builders::{GuestBuilder, UpdateGuestBuilder},
    client::{patch_json, post_json, put_json, response_json},
};

pub async fn create_guest(app: &Router) -> GuestResponse {
    let request: CreateGuestRequest = GuestBuilder::new().build();

    let response = post_json(app, "/guests", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED,);

    let body = response_json(response).await;

    serde_json::from_value::<GuestResponse>(body).unwrap()
}

pub async fn update_guest(app: &Router, guest_id: uuid::Uuid) -> GuestResponse {
    let request: UpdateGuestRequest = UpdateGuestBuilder::new().build();

    let response = patch_json(app, &format!("/guests/{}", guest_id,), &request).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let body = response_json(response).await;

    serde_json::from_value::<GuestResponse>(body).unwrap()
}
