use crate::helpers::{
    app::test_app,

    guest::{
        create_guest_with,
    },

    reservation::{
        create_reservation_with,
    },

    builders::{
        guest_builder::
            GuestBuilder,

        reservation_request_builder::
            ReservationRequestBuilder,
    },
};

use pms_rs::projection::rebuild::
    reservation_search_rebuild::
    rebuild_reservation_search_projection;

use pms_rs::projection::service::
    reservation_search_projection_service::
    refresh_reservation_search_projection;

use pms_rs::repository::sqlite::projection::
    reservation_search_projection_repository::
    ReservationSearchProjectionRepository;

#[tokio::test]
async fn should_rebuild_reservation_search_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Sato",
                    "Hanako",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "rebuild-1",
                )
                .with_room_class(
                    "TWIN",
                )
                .with_primary_guest(
                    guest_id,
                )
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    rebuild_reservation_search_projection(
        &mut tx,
    )
    .await
    .unwrap();

    let projection =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        projection.reservation_id,
        reservation_id,
    );

    assert_eq!(
        projection.external_id,
        "rebuild-1",
    );

    assert_eq!(
        projection.room_class,
        "TWIN",
    );

    assert_eq!(
        projection.primary_guest_name,
        "Sato Hanako",
    );

    assert_eq!(
        projection.participant_names.len(),
        1,
    );

    tx.rollback()
        .await
        .unwrap();
}

#[tokio::test]
async fn should_match_refresh_and_rebuild_reservation_projection() {

    let app =
        test_app().await;

    let primary_guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Kobayashi",
                    "Taro",
                )
        )
        .await;

    let accompany_guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Kobayashi",
                    "Hanako",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "family-1",
                )
                .with_room_class(
                    "FAMILY",
                )
                .with_primary_guest(
                    primary_guest_id,
                )
                .with_accompany_guest(
                    accompany_guest_id,
                )
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    refresh_reservation_search_projection(
        &mut tx,
        reservation_id,
    )
    .await
    .unwrap();

    let refreshed =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    ReservationSearchProjectionRepository
        ::delete_all(
            &mut tx,
        )
        .await
        .unwrap();

    rebuild_reservation_search_projection(
        &mut tx,
    )
    .await
    .unwrap();

    let rebuilt =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        refreshed.reservation_id,
        rebuilt.reservation_id,
    );

    assert_eq!(
        refreshed.external_id,
        rebuilt.external_id,
    );

    assert_eq!(
        refreshed.primary_guest_name,
        rebuilt.primary_guest_name,
    );

    assert_eq!(
        refreshed.participant_names,
        rebuilt.participant_names,
    );

    assert_eq!(
        refreshed.check_in,
        rebuilt.check_in,
    );

    assert_eq!(
        refreshed.check_out,
        rebuilt.check_out,
    );

    assert_eq!(
        refreshed.room_class,
        rebuilt.room_class,
    );

    assert_eq!(
        refreshed.room_id,
        rebuilt.room_id,
    );

    assert_eq!(
        refreshed.reservation_status,
        rebuilt.reservation_status,
    );

    assert_eq!(
        refreshed.stay_status,
        rebuilt.stay_status,
    );

    assert_eq!(
        refreshed.projection_version,
        rebuilt.projection_version,
    );

    tx.rollback()
        .await
        .unwrap();
}