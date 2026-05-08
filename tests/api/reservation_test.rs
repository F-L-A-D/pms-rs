use chrono::NaiveDate;

use uuid::Uuid;

use pms_rs::db::connection::Db;

use pms_rs::domain::guest::Guest;

use pms_rs::domain::reservation::Reservation;

use pms_rs::domain::reservation_guest_relation::{
    ReservationGuestRelation,
    ReservationGuestRelationType,
};

use pms_rs::repository::sqlite::operational::
    reservation_repository::SqliteReservationRepository;

use pms_rs::usecase::guest::
    create_guest::create_guest;

use pms_rs::usecase::reservation::
    create_reservation::create_reservation;

#[tokio::test]
async fn should_create_reservation_with_primary_participant() {

    let db =
        Db::new("sqlite::memory:")
            .await;

    let guest_id = Uuid::new_v4();

    let guest =
        Guest::new(
            guest_id,
            "Yamada".into(),
            "Taro".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

    create_guest(
        &db,
        guest,
    )
    .await
    .unwrap();

    let reservation =
        Reservation::new(
            "res-1".into(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                1,
            )
            .unwrap(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                3,
            )
            .unwrap(),
            "DOUBLE".into(),
            vec![
                ReservationGuestRelation {
                    reservation_id:
                        "res-1".into(),

                    guest_id:
                        guest_id,

                    relation_type:
                        ReservationGuestRelationType::Primary,
                }
            ],
        )
        .unwrap();

    create_reservation(
        &db,
        reservation.clone(),
    )
    .await
    .unwrap();

    let mut tx =
        db.begin_tx().await;

    let loaded =
        SqliteReservationRepository
            ::find_by_id(
                &mut tx,
                "res-1",
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        loaded.participants.len(),
        1,
    );

    assert_eq!(
        loaded.participants[0]
            .guest_id,
        guest_id,
    );

    assert!(
        loaded.participants[0]
            .is_primary()
    );
}

#[tokio::test]
async fn should_fail_when_duplicate_participants_exist() {

    let guest_id = Uuid::new_v4();

    let reservation =
        Reservation::new(
            "res-1".into(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                1,
            )
            .unwrap(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                2,
            )
            .unwrap(),
            "DOUBLE".into(),
            vec![
                ReservationGuestRelation {
                    reservation_id:
                        "res-1".into(),

                    guest_id:
                        guest_id,

                    relation_type:
                        ReservationGuestRelationType::Primary,
                },

                ReservationGuestRelation {
                    reservation_id:
                        "res-1".into(),

                    guest_id:
                        guest_id,

                    relation_type:
                        ReservationGuestRelationType::Accompany,
                }
            ],
        );

    assert!(
        reservation.is_err()
    );
}

#[tokio::test]
async fn should_fail_when_primary_participant_missing() {

    let guest_id = Uuid::new_v4();

    let reservation =
        Reservation::new(
            "res-1".into(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                1,
            )
            .unwrap(),
            NaiveDate::from_ymd_opt(
                2026,
                1,
                2,
            )
            .unwrap(),
            "DOUBLE".into(),
            vec![
                ReservationGuestRelation {
                    reservation_id:
                        "res-1".into(),

                    guest_id:
                        guest_id,

                    relation_type:
                        ReservationGuestRelationType::Accompany,
                }
            ],
        );

    assert!(
        reservation.is_err()
    );
}

#[tokio::test]
async fn should_find_reservations_by_guest_id() {

    let db =
        Db::new("sqlite::memory:")
            .await;
    
    let guest_id = Uuid::new_v4();

    let guest =
        Guest::new(
            guest_id,
            "Suzuki".into(),
            "Hanako".into(),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
        )
        .unwrap();

    create_guest(
        &db,
        guest,
    )
    .await
    .unwrap();

    let reservation =
        Reservation::new(
            "res-1".into(),
            NaiveDate::from_ymd_opt(
                2026,
                2,
                1,
            )
            .unwrap(),
            NaiveDate::from_ymd_opt(
                2026,
                2,
                3,
            )
            .unwrap(),
            "TWIN".into(),
            vec![
                ReservationGuestRelation {
                    reservation_id:
                        "res-1".into(),

                    guest_id:
                        guest_id,

                    relation_type:
                        ReservationGuestRelationType::Primary,
                }
            ],
        )
        .unwrap();

    create_reservation(
        &db,
        reservation,
    )
    .await
    .unwrap();

    let mut tx =
        db.begin_tx().await;

    let reservations =
        SqliteReservationRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap();

    assert_eq!(
        reservations.len(),
        1,
    );

    assert_eq!(
        reservations[0].id,
        "res-1",
    );
}