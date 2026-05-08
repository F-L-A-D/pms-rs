use pms_rs::repository::sqlite::projection::
    guest_summary_projection_repository::
    GuestSummaryProjectionRepository;

use crate::helpers::{
    app::test_app,
    guest::create_guest,
    projection::rebuild_guest_summary,
};

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