use chrono::NaiveDate;

use pms_rs::db::connection::Db;

use pms_rs::domain::guest::Guest;

use pms_rs::adapter::stay_input::StayInput;

use pms_rs::repository::sqlite::guest_repository::SqliteGuestRepository;
use pms_rs::repository::sqlite::reservation_repository::SqliteReservationRepository;

use pms_rs::usecase::reservation::create_reservation::create;

#[tokio::test]
async fn should_create_reservation_with_primary_guest() {

    let db =
        Db::new("sqlite::memory:")
            .await;

    let guest =
        Guest::new(
            "guest-001".into(),
            "Yamada".into(),
            "Taro".into(),
            None,
            None,
        )
        .unwrap();

    SqliteGuestRepository::save(
        &db.pool,
        &guest,
    )
    .await
    .unwrap();

    let check_in =
        NaiveDate::from_ymd_opt(
            2026,
            5,
            1,
        )
        .unwrap();

    create(
        &db,
        "reservation-001".into(),
        StayInput::CheckInAndNights {
            check_in,
            nights: 2,
            room_class: "STD".into(),
        },
        Some("guest-001".into()),
    )
    .await
    .unwrap();

    let mut tx = db.begin_tx().await;

    let reservation =
        SqliteReservationRepository::find_by_id_tx(
            &mut tx,
            "reservation-001",
        )
        .await
        .unwrap()
        .unwrap();

    tx.rollback().await.unwrap();

    assert_eq!(
        reservation.primary_guest_id,
        Some("guest-001".into()),
    );
}

#[tokio::test]
async fn should_fail_create_reservation_with_nonexistent_guest() {

    let db =
        Db::new("sqlite::memory:")
            .await;

    let check_in =
        NaiveDate::from_ymd_opt(
            2026,
            5,
            1,
        )
        .unwrap();

    let result =
        create(
            &db,
            "reservation-001".into(),
            StayInput::CheckInAndNights {
                check_in,
                nights: 2,
                room_class: "STD".into(),
            },
            Some("guest-999".into()),
        )
        .await;

    assert!(
        result.is_err()
    );
}