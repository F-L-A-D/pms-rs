use axum::{http::StatusCode, Router};

use chrono::NaiveDate;

use serde_json::{json, Value};

use super::client::{get, post_json, response_json};

pub async fn get_current_business_date(app: &Router) -> Value {
    let response = get(app, "/business-date/current").await;

    assert_eq!(response.status(), StatusCode::OK);

    response_json(response).await
}

pub async fn current_open_business_date(app: &Router) -> NaiveDate {
    let current = get_current_business_date(app).await;

    assert_eq!(current["status"], "open");

    let business_date = current["business_date"].as_str().unwrap();

    NaiveDate::parse_from_str(business_date, "%Y-%m-%d").unwrap()
}

pub async fn start_night_audit(app: &Router) -> Value {
    let response = post_json(
        app,
        "/business-date/night-audit/start",
        &json!({ "reason": "test night audit" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    response_json(response).await
}

pub async fn finalize_night_audit(app: &Router) -> Value {
    let response = post_json(
        app,
        "/business-date/night-audit/finalize",
        &json!({ "reason": "test night audit" }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    response_json(response).await
}

pub async fn get_night_audit_worklist(app: &Router) -> Value {
    let response = get(app, "/business-date/night-audit/worklist").await;

    assert_eq!(response.status(), StatusCode::OK);

    response_json(response).await
}

pub async fn post_night_audit_room_charges(app: &Router) -> Value {
    let response = post_json(
        app,
        "/business-date/night-audit/post-room-charges",
        &json!({}),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    response_json(response).await
}

pub async fn advance_business_date(app: &Router) -> Value {
    start_night_audit(app).await;
    post_night_audit_room_charges(app).await;
    finalize_night_audit(app).await
}
