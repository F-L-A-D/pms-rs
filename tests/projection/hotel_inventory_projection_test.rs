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
    hotel_inventory_projection_repository::
        SqliteHotelInventoryProjectionRepository;

#[tokio::test]
async fn should_aggregate_inventory_across_room_classes() {

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

    let inventory =
        SqliteHotelInventoryProjectionRepository
            ::find_by_date(
                &mut tx,
                &date,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        inventory.reserved_rooms,
        2,
    );
}