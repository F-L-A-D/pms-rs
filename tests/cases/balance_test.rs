use chrono::NaiveDate;

use pms_rs::db::connection::Db;

use pms_rs::adapter::stay_input::StayInput;

use pms_rs::usecase::reservation::create::create;

use pms_rs::usecase::billing::open_folio::open_folio;
use pms_rs::usecase::billing::calculate_balance::calculate_balance;

use pms_rs::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;

use pms_rs::domain::folio_entry::{
    FolioEntry,
    EntryType,
};

#[tokio::test]
async fn should_return_zero_for_empty_folio() {

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

    let balance =
        calculate_balance(
            &db,
            "f1",
        )
        .await
        .unwrap();

    assert_eq!(balance, 0);
}

#[tokio::test]
async fn should_calculate_positive_balance() {

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

    let charge = FolioEntry::new(
        "e1".into(),
        "f1".into(),
        EntryType::RoomCharge,
        12000,
        None,
    );

    SqliteFolioEntryRepository::save(
        &db.pool,
        &charge,
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
async fn should_calculate_negative_balance() {

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

    let payment = FolioEntry::new(
        "e1".into(),
        "f1".into(),
        EntryType::Payment,
        -5000,
        None,
    );

    SqliteFolioEntryRepository::save(
        &db.pool,
        &payment,
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

    assert_eq!(balance, -5000);
}

#[tokio::test]
async fn should_calculate_mixed_balance() {

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

    let charge = FolioEntry::new(
        "e1".into(),
        "f1".into(),
        EntryType::RoomCharge,
        12000,
        None,
    );

    let payment = FolioEntry::new(
        "e2".into(),
        "f1".into(),
        EntryType::Payment,
        -5000,
        None,
    );

    SqliteFolioEntryRepository::save(
        &db.pool,
        &charge,
    )
    .await
    .unwrap();

    SqliteFolioEntryRepository::save(
        &db.pool,
        &payment,
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

    assert_eq!(balance, 7000);
}