use chrono::NaiveDate;

use serde_json::json;

use crate::helpers::{
    app::test_app,
    guest::create_guest,
    reservation::{
        create_reservation,
        modify_reservation,
    },
};

use pms_rs::repository::sqlite::projection::
    inventory_projection_repository::
        SqliteInventoryProjectionRepository;

#[tokio::test]
async fn should_separate_inventory_by_room_class() {

    let app =
        test_app().await;

    let guest1 =
        create_guest(&app.app).await;

    let guest2 =
        create_guest(&app.app).await;

    let res1 = 
        create_reservation(
            &app.app,
            "res-1",
            guest1,
        )
        .await;

    let _res2 = 
        create_reservation(
            &app.app,
            "res-2",
            guest2,
        )
        .await;

    modify_reservation(
        &app.app,
        res1,
        json!({
        "room_class": "DLX"
    }),
    )
    .await;

    let mut tx =
        app.db.begin_tx().await;

    let date =
        NaiveDate::from_ymd_opt(
            2026,
            5,
            10,
        )
        .unwrap()
        .to_string();

    let std_inventory =
        SqliteInventoryProjectionRepository
            ::find_by_date_and_room_class(
                &mut tx,
                &date,
                "STD",
            )
            .await
            .unwrap()
            .unwrap();

    let dlx_inventory =
        SqliteInventoryProjectionRepository
            ::find_by_date_and_room_class(
                &mut tx,
                &date,
                "DLX",
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        std_inventory.reserved_rooms,
        1,
    );

    assert_eq!(
        dlx_inventory.reserved_rooms,
        1,
    );
}

#[tokio::test]
async fn should_transition_inventory_between_room_classes() {

    let app =
        test_app().await;

    let guest =
        create_guest(&app.app).await;

    let reservation =
        create_reservation(
            &app.app,
            "res-1",
            guest,
        )
        .await;

    modify_reservation(
        &app.app,
        reservation,
        json!({
            "room_class": "DLX"
        }),
    )
    .await;

    let mut tx =
        app.db.begin_tx().await;

    let date =
        NaiveDate::from_ymd_opt(
            2026,
            5,
            10,
        )
        .unwrap()
        .to_string();

    let std_inventory =
        SqliteInventoryProjectionRepository
            ::find_by_date_and_room_class(
                &mut tx,
                &date,
                "STD",
            )
            .await
            .unwrap();

    let dlx_inventory =
        SqliteInventoryProjectionRepository
            ::find_by_date_and_room_class(
                &mut tx,
                &date,
                "DLX",
            )
            .await
            .unwrap()
            .unwrap();
        
    assert_eq!(
        std_inventory.unwrap().reserved_rooms,
        0,
    );

    assert_eq!(
        dlx_inventory.reserved_rooms,
        1,
    );
}