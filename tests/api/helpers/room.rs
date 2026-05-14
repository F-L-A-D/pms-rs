use axum::{
    http::StatusCode,
    Router,
};

use pms_rs::api::dto::room::RoomResponse;

use crate::api::helpers::{
    builders::RoomBuilder,

    client::{
        post_json,
        response_json,
    },
};

pub async fn create_room(
    app: &Router
) -> RoomResponse {

    let request =
        RoomBuilder::new()
            .build();
    
    let response = 
        post_json(
            app, 
            "/rooms", 
            &request,
        )
        .await;

    
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
    );

    serde_json::from_value(
        response_json(response)
            .await,
    )
    .unwrap()

}