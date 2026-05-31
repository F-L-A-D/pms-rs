use axum::{http::StatusCode, Router};

use chrono::NaiveDate;

use serde_json::json;

use uuid::Uuid;

use super::client::{post_json, response_json};

pub async fn inspect_room_for_date(app: &Router, room_id: Uuid, service_date: NaiveDate) {
    let body = json!({ "service_date": service_date.to_string() });

    let response = post_json(app, &format!("/housekeeping/{}/dirty", room_id), &body).await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = post_json(
        app,
        &format!("/housekeeping/{}/start-cleaning", room_id),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = post_json(
        app,
        &format!("/housekeeping/{}/finish-cleaning", room_id),
        &body,
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = post_json(app, &format!("/housekeeping/{}/inspect", room_id), &body).await;
    assert_eq!(response.status(), StatusCode::OK);

    let body_json = response_json(response).await;
    assert_eq!(body_json["housekeeping_status"], "inspected");
}
