use pms_rs::repository::sqlite::projection::
    guest_summary_projection_repository::
    GuestSummaryProjectionRepository;

use crate::helpers::{
    app::test_app,
    guest::create_guest,
    projection::
        materialize_guest_summary_projection,
};

#[tokio::test]
async fn should_materialize_guest_summary_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    let projection =
        materialize_guest_summary_projection(
            &app.db,
            guest_id,
        )
        .await;

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

    assert_eq!(
        projection.projection_version,
        1,
    );
}

#[tokio::test]
async fn should_persist_guest_summary_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    let projection =
        materialize_guest_summary_projection(
            &app.db,
            guest_id,
        )
        .await;

    let mut tx =
        app.db.begin_tx().await;

    GuestSummaryProjectionRepository
        ::upsert(
            &mut tx,
            &projection,
        )
        .await
        .unwrap();

    let loaded =
        GuestSummaryProjectionRepository
            ::find_by_guest_id(
                &mut tx,
                guest_id,
            )
            .await
            .unwrap()
            .unwrap();

    assert_eq!(
        loaded.guest_id,
        guest_id,
    );

    assert_eq!(
        loaded.total_stays,
        0,
    );
}