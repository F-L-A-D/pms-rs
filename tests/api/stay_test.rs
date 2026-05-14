use crate::api::helpers::{
    app::spawn_app,

    reservation::create_reservation,

    room::create_room,

    stay::{
        assign_room, check_in, check_out
    },
};

#[tokio::test]
async fn should_chek_in_reservation() {

    let app = spawn_app().await;

    let reservation = 
        create_reservation(&app.app)
            .await;
    
    let room = 
        create_room(&app.app)
            .await;
    
    assign_room(
        &app.app, 
        reservation.id, 
        room.id,
    )
    .await;
    
    let response = 
        check_in(
            &app.app, 
            reservation.id
        )
        .await;
    
    assert_eq!(
        response.id,
        reservation.id,
    );

    assert_eq!(
        response.status,
        "checked_in",
    );
}

#[tokio::test]
async fn should_chek_out_reservation() {

    let app = spawn_app().await;

    let reservation = 
        create_reservation(&app.app)
            .await;

    let room = 
        create_room(&app.app)
            .await;
    
    assign_room(
        &app.app, 
        reservation.id, 
        room.id,
    )
    .await;
    
    check_in(
        &app.app, 
        reservation.id
    )
    .await;

    let response = 
        check_out(
            &app.app, 
            reservation.id
        )
        .await;
    
    assert_eq!(
        response.id,
        reservation.id,
    );

    assert_eq!(
        response.status,
        "checked_out",
    );
}