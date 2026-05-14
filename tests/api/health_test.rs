use axum::{
    body::Body,
    http::{
        Request,
        StatusCode,
    },
};

use tower::ServiceExt;

use crate::api::helpers::app::spawn_app;

#[tokio::test]
async fn should_health_returns_ok() {

    let app = spawn_app().await;

    let response =
        app.app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );
}