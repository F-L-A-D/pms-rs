use chrono::NaiveDate;

use pms_rs::db::connection::Db;

use pms_rs::adapter::stay_input::StayInput;

use pms_rs::usecase::reservation::create::create;

use pms_rs::usecase::billing::open_folio::open_folio;
use pms_rs::usecase::billing::calculate_balance::calculate_balance;
use pms_rs::usecase::billing::post_room_charge::post_room_charge;

#[tokio::test]
async fn should_post_room_charge() {

    let db = Db::new("sqlite::memory:").await;

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into(),
        },
    )
    .await
    .unwrap();

    open_folio(
        &db,
        "f1".into(),
        "r1".into(),
    )
    .await
    .unwrap();

    let result =
        post_room_charge(
            &db,
            "e1".into(),
            "f1",
            12000,
            Some("room charge".into()),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_update_balance_after_room_charge() {

    let db = Db::new("sqlite::memory:").await;

    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
            room_class: "single".into(),
        },
    )
    .await
    .unwrap();

    open_folio(
        &db,
        "f1".into(),
        "r1".into(),
    )
    .await
    .unwrap();

    post_room_charge(
        &db,
        "e1".into(),
        "f1",
        12000,
        None,
    )
    .await
    .unwrap();

    let balance =
        calculate_balance(
            &db,
            "f1",
        )
        .await
        .unwrap();

    assert_eq!(balance, 12000);
}

#[tokio::test]
async fn should_fail_when_folio_not_found() {

    let db = Db::new("sqlite::memory:").await;

    let result =
        post_room_charge(
            &db,
            "e1".into(),
            "missing",
            12000,
            None,
        )
        .await;

    assert!(result.is_err());
}