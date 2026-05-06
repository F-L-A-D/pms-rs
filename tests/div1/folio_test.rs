use chrono::NaiveDate;

use pms_rs::db::connection::Db;

use pms_rs::adapter::stay_input::StayInput;

use pms_rs::usecase::reservation::create::create;

use pms_rs::usecase::billing::open_folio::open_folio;
use pms_rs::usecase::billing::close_folio::close_folio;

use pms_rs::repository::sqlite::folio_repository::SqliteFolioRepository;

use pms_rs::domain::folio::FolioStatus;

#[tokio::test]
async fn should_open_folio() {

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

    let result = open_folio(
        &db,
        "f1".into(),
        "r1".into(),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_close_folio() {

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

    close_folio(&db, "f1")
        .await
        .unwrap();

    let folio =
        SqliteFolioRepository::find_by_id(
            &db.pool,
            "f1",
        )
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        folio.status,
        FolioStatus::Closed
    );
}

#[tokio::test]
async fn should_fail_double_close() {

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

    close_folio(&db, "f1")
        .await
        .unwrap();

    let result = close_folio(&db, "f1").await;

    assert!(result.is_err());
}