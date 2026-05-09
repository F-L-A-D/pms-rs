use crate::helpers::{
    app::test_app,

    guest::{
        create_guest_with,
    },

    reservation::{
        create_reservation_with,
        modify_reservation,
    },

    builders::{
        guest_builder::
            GuestBuilder,

        reservation_request_builder::
            ReservationRequestBuilder,

        reservation_modify_builder::
            ReservationModifyBuilder,
    },
};

use pms_rs::projection::materializer::
    reservation_search_materializer::
    materialize_reservation_search_projection;

use pms_rs::projection::service::
    reservation_search_projection_service::
    refresh_reservation_search_projection;

use pms_rs::repository::sqlite::projection::
    reservation_search_projection_repository::
    ReservationSearchProjectionRepository;

#[tokio::test]
async fn should_materialize_reservation_search_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Yamada",
                    "Taro",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "res-1",
                )
                .with_room_class(
                    "STD",
                )
                .with_primary_guest(
                    guest_id,
                )
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    let projection =
        materialize_reservation_search_projection(
            &mut tx,
            reservation_id,
        )
        .await
        .unwrap();

    assert_eq!(
        projection.reservation_id,
        reservation_id,
    );

    assert_eq!(
        projection.external_id,
        "res-1",
    );

    assert_eq!(
        projection.room_class,
        "STD",
    );

    assert_eq!(
        projection.participant_names.len(),
        1,
    );

    assert_eq!(
        projection.primary_guest_name,
        "Yamada Taro",
    );

    tx.rollback()
        .await
        .unwrap();
}

#[tokio::test]
async fn should_persist_reservation_search_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Suzuki",
                    "Hanako",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "persist-1",
                )
                .with_room_class(
                    "DLX",
                )
                .with_primary_guest(
                    guest_id,
                )
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    let projection =
        materialize_reservation_search_projection(
            &mut tx,
            reservation_id,
        )
        .await
        .unwrap();

    ReservationSearchProjectionRepository
        ::upsert(
            &mut tx,
            &projection,
        )
        .await
        .unwrap();

    let loaded =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        loaded.reservation_id,
        reservation_id,
    );

    assert_eq!(
        loaded.external_id,
        "persist-1",
    );

    assert_eq!(
        loaded.room_class,
        "DLX",
    );

    assert_eq!(
        loaded.primary_guest_name,
        projection.primary_guest_name,
    );

    assert_eq!(
        loaded.participant_names,
        projection.participant_names,
    );

    assert_eq!(
        loaded.projection_version,
        1,
    );

    tx.rollback()
        .await
        .unwrap();
}

#[tokio::test]
async fn should_delete_reservation_search_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Tanaka",
                    "Ichiro",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "delete-1",
                )
                .with_primary_guest(
                    guest_id,
                )
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    let projection =
        materialize_reservation_search_projection(
            &mut tx,
            reservation_id,
        )
        .await
        .unwrap();

    ReservationSearchProjectionRepository
        ::upsert(
            &mut tx,
            &projection,
        )
        .await
        .unwrap();

    ReservationSearchProjectionRepository
        ::delete_by_reservation_id(
            &mut tx,
            reservation_id,
        )
        .await
        .unwrap();

    let loaded =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap();

    assert!(
        loaded.is_none(),
    );

    tx.rollback()
        .await
        .unwrap();
}

#[tokio::test]
async fn should_find_reservation_by_partial_guest_name() {

    let app =
        test_app().await;

    let target_guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Yamada",
                    "Taro",
                )
        )
        .await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Yamamoto",
                "Jiro",
            )
    )
    .await;

    create_guest_with(
        &app.app,

        GuestBuilder::new()
            .with_name(
                "Tanaka",
                "Saburo",
            )
    )
    .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "search-1",
                )
                .with_room_class(
                    "STD",
                )
                .with_primary_guest(
                    target_guest_id,
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

    let projections =
        ReservationSearchProjectionRepository
            ::find_by_guest_name_partial(
                &mut tx,
                "Yamada",
            )
            .await
            .unwrap();

    assert_eq!(
        projections.len(),
        1,
    );

    assert_eq!(
        projections[0]
            .reservation_id,
        reservation_id,
    );

    assert_eq!(
        projections[0]
            .external_id,
        "search-1",
    );

    assert_eq!(
        projections[0]
            .primary_guest_name,
        "Yamada Taro",
    );

    tx.rollback()
        .await
        .unwrap();
}

#[tokio::test]
async fn should_refresh_projection_after_room_class_modified() {

    let app =
        test_app().await;

    let guest_id =
        create_guest_with(
            &app.app,

            GuestBuilder::new()
                .with_name(
                    "Sato",
                    "Taro",
                )
        )
        .await;

    let reservation_id =
        create_reservation_with(
            &app.app,

            ReservationRequestBuilder::new()
                .with_external_id(
                    "modify-1",
                )
                .with_room_class(
                    "STD",
                )
                .with_primary_guest(
                    guest_id,
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

    let before =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        before.room_class,
        "STD",
    );

    tx.rollback()
        .await
        .unwrap();

    modify_reservation(
        &app.app,

        reservation_id,

        ReservationModifyBuilder::new()
            .with_room_class(
                "DLX",
            )
            .build(),
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

    let after =
        ReservationSearchProjectionRepository
            ::find_by_reservation_id(
                &mut tx,
                reservation_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        after.room_class,
        "DLX",
    );

    tx.rollback()
        .await
        .unwrap();
}