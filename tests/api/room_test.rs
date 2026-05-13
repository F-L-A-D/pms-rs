use axum::http::StatusCode;

use pms_rs::api::dto::room::RoomResponse;

use crate::api::helpers::{
    app::spawn_app,

    room::create_room,
};

#[tokio::test]
async fn shoud_create_room() {

    let app = spawn_app().await;

    let room = 
        create_room(&app.app)
            .await;
    
    assert_eq!(
        room.room_class,
        "standard",
    );

    assert_eq!(
        room.occupancy_status,
        "Vacant",
    );

    assert_eq!(
        room.housekeeping_status,
        "Inspected",
    );
}

