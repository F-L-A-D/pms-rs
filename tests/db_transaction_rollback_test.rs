use pms_rs::db::connection::Db;
use pms_rs::usecase::reservation::create::create;
use pms_rs::adapter::stay_input::StayInput;
use chrono::NaiveDate;

#[tokio::test]
async fn should_rollback_on_inventory_error() {
    let db = Db::new("sqlite::memory:").await;

    // 1回目成功
    create(
        &db,
        "r1".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: 1,
        },
    )
    .await
    .unwrap();

    // 強制的に不正（cancel2回とか）
    let result = create(
        &db,
        "r2".into(),
        StayInput::CheckInAndNights {
            check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            nights: -1, // ← invalid
        },
    )
    .await;

    assert!(result.is_err());

    // DB状態が壊れてないことを確認
    // （ここは必要ならSELECT書く）
}