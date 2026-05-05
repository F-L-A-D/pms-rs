use chrono::NaiveDate;

use pms_rs::db::connection::Db;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::usecase::reservation::create::create;
use pms_rs::adapter::stay_input::StayInput;
use pms_rs::repository::sqlite::repository::SqliteReservationRepository;

#[tokio::test]
async fn transaction_should_rollback_on_error() {
    let db = Db::new("sqlite::memory:").await;
    let mut inv = HotelInventory::new(10);

    let result = create(
        &db,
        &mut inv,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: -1, // invalid
        },
    )
    .await;

    assert!(result.is_err());

    let mut tx = db.begin_tx().await;

    let res = SqliteReservationRepository::find_by_id_tx(&mut tx, "r1")
        .await
        .unwrap();

    assert!(res.is_none());
}