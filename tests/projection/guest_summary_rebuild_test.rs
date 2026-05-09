use pms_rs::repository::sqlite::projection::
    guest_summary_projection_repository::
    GuestSummaryProjectionRepository;

use crate::helpers::{
    app::test_app,
    guest::create_guest,
    reservation::create_reservation,
    projection::rebuild_guest_summary,
};

use pms_rs::projection::rebuild::
    guest_summary_rebuild::
    rebuild_guest_summary_projection;

#[tokio::test]
async fn should_rebuild_guest_summary_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    rebuild_guest_summary(
        &app.db,
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
        0,
    );

    assert_eq!(
        projection.total_nights,
        0,
    );

    assert_eq!(
        projection.total_spending,
        0,
    );

    assert_eq!(
        projection.last_stay_at,
        None,
    );
}

#[tokio::test]
async fn should_match_refresh_and_rebuild_projection() {

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

    let refreshed =
        GuestSummaryProjectionRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap()
            .unwrap();

    GuestSummaryProjectionRepository
        ::delete_all(
            &mut tx,
        )
        .await
        .unwrap();

    rebuild_guest_summary_projection(
        &mut tx,
    )
    .await
    .unwrap();

    let rebuilt =
        GuestSummaryProjectionRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        refreshed.guest_id,
        rebuilt.guest_id,
    );

    assert_eq!(
        refreshed.total_stays,
        rebuilt.total_stays,
    );

    assert_eq!(
        refreshed.total_nights,
        rebuilt.total_nights,
    );

    assert_eq!(
        refreshed.total_spending,
        rebuilt.total_spending,
    );

    assert_eq!(
        refreshed.last_stay_at,
        rebuilt.last_stay_at,
    );

    assert_eq!(
        refreshed.projection_version,
        rebuilt.projection_version,
    );
}