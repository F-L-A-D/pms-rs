use axum::{
    http::StatusCode,
    Router,
};

use uuid::Uuid;

use pms_rs::api::dto::stay::StayResponse;

use crate::api::helpers::client::{
    post,
    post_json, 
    response_json,
};

pub async fn assign_room(
    app: &Router,
    reservation_id: Uuid,
    room_id: Uuid,
) -> StayResponse {
    
    let response = 
        post(
            app,
            &format!(
                "/reservations/{}/assign-room/{}",
                reservation_id,
                room_id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    serde_json::from_value(
        response_json(response)
            .await,
    )
    .unwrap()
}

pub async fn check_in(
    app: &Router,
    reservation_id: Uuid,
) -> StayResponse {
    
    let response = 
        post(
            app,
            &format!(
                "/reservations/{}/check-in",
                reservation_id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    serde_json::from_value(
        response_json(response)
            .await,
    )
    .unwrap()
}

pub async fn check_out(
    app: &Router,
    reservation_id: Uuid,
) -> StayResponse {
    
    let response = 
        post(
            app,
            &format!(
                "/reservations/{}/check-out",
                reservation_id,
            ),
        )
        .await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
    );

    serde_json::from_value(
        response_json(response)
            .await,
    )
    .unwrap()
}