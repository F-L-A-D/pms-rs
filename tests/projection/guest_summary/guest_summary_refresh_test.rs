use pms_rs::repository::sqlite::projection::
    guest_summary_projection_repository::
    GuestSummaryProjectionRepository;

use crate::helpers::{
    app::test_app,
    billing::{
        open_folio,
        post_room_charge,
    },
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_refresh_projection_after_reservation_created() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    create_reservation(
        &app.app,
        "reservation-001",
        guest_id,
    )
    .await;

    let mut tx =
        app.db.begin_tx().await;

    let projection =
        GuestSummaryProjectionRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        projection.guest_id,
        guest_id,
    );

    assert_eq!(
        projection.total_stays,
        1,
    );

    assert_eq!(
        projection.total_nights,
        2,
    );

    assert_eq!(
        projection.total_spending,
        0,
    );

    assert_eq!(
        projection.projection_version,
        1,
    );
}

#[tokio::test]
async fn should_refresh_total_spending_after_room_charge() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    let reservation_id =
        create_reservation(
            &app.app,
            "reservation-001",
            guest_id,
        )
        .await;

    let folio_id = 
        open_folio(
            &app.app,
            reservation_id,
        )
        .await;

    post_room_charge(
        &app.app,
        folio_id,
        12000,
    )
    .await;

    let mut tx =
        app.db.begin_tx().await;

    let projection =
        GuestSummaryProjectionRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        projection.total_spending,
        12000,
    );
}