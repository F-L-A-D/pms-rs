use chrono::NaiveDate;

use pms_rs::db::connection::Db;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::usecase::reservation::create::create;
use pms_rs::usecase::reservation::cancel::cancel;
use pms_rs::adapter::stay_input::StayInput;
use pms_rs::repository::sqlite::repository::SqliteReservationRepository;

#[tokio::test]
async fn cancel_should_update_status() {
    let db = Db::new("sqlite::memory:").await;
    let mut inv = HotelInventory::new(10);

    create(
        &db,
        &mut inv,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 2,
        },
    )
    .await
    .unwrap();

    cancel(&db, &mut inv, "r1")
        .await
        .unwrap();

    let mut tx = db.begin_tx().await;

    let res = SqliteReservationRepository::find_by_id_tx(&mut tx, "r1")
        .await
        .unwrap()
        .unwrap();

    assert_eq!(format!("{:?}", res.status), "Cancelled");
}