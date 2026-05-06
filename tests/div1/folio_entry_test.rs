use chrono::NaiveDate;

use pms_rs::db::connection::Db;
use pms_rs::adapter::stay_input::StayInput;
use pms_rs::usecase::reservation::create::create;
use pms_rs::usecase::billing::open_folio::open_folio;
use pms_rs::repository::sqlite::folio_entry_repository::SqliteFolioEntryRepository;

use pms_rs::domain::folio_entry::{
    FolioEntry,
    EntryType,
};

#[tokio::test]
async fn should_save_folio_entry() {

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

    let entry = FolioEntry::new(
        "e1".into(),
        "f1".into(),
        EntryType::RoomCharge,
        12000,
        Some("room charge".into()),
    );

    let result =
        SqliteFolioEntryRepository::save(
            &db.pool,
            &entry,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_find_entries_by_folio_id() {

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

    let e1 = FolioEntry::new(
        "e1".into(),
        "f1".into(),
        EntryType::RoomCharge,
        12000,
        None,
    );

    let e2 = FolioEntry::new(
        "e2".into(),
        "f1".into(),
        EntryType::Payment,
        -12000,
        None,
    );

    SqliteFolioEntryRepository::save(
        &db.pool,
        &e1,
    )
    .await
    .unwrap();

    SqliteFolioEntryRepository::save(
        &db.pool,
        &e2,
    )
    .await
    .unwrap();

    let entries =
        SqliteFolioEntryRepository::find_by_folio_id(
            &db.pool,
            "f1",
        )
        .await
        .unwrap();

    assert_eq!(entries.len(), 2);
}