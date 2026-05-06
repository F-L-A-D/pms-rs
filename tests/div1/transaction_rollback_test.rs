#![allow(unused_imports)]
use pms_rs::db::connection::Db;
use pms_rs::usecase::reservation::create::create;
use pms_rs::adapter::stay_input::StayInput;
use chrono::NaiveDate;

#[tokio::test]
async fn should_rollback_on_inventory_error() {
    let db = Db::new("sqlite::memory:").await;

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into()
        },
    )
    .await
    .unwrap();

    let result = create(
        &db,
        "r2".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: -1,
            room_class: "single".into()
        },
    )
    .await;

    assert!(result.is_err());
}