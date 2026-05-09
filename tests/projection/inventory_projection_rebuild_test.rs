use crate::helpers::{
    app::test_app,
    guest::create_guest,
    reservation::create_reservation,
};

use pms_rs::repository::sqlite::projection::
    inventory_projection_repository::
        SqliteInventoryProjectionRepository;

use pms_rs::projection::{
    rebuild::inventory_projection_rebuild::
        rebuild_inventory_projection,

    materializer::inventory_materializer::
        materialize_hotel_inventory,
};

#[tokio::test]
async fn should_match_refresh_and_rebuild_inventory_projection() {

    let app =
        test_app().await;

    let guest_id =
        create_guest(&app.app).await;

    create_reservation(
        &app.app,
        "res-1",
        guest_id,
    )
    .await;

    let mut tx =
        app.db.begin_tx().await;

    let before =
        SqliteInventoryProjectionRepository
            ::list_all(
                &mut tx,
            )
            .await
            .unwrap();

    let before_view =
        materialize_hotel_inventory(
            before,
        );

    rebuild_inventory_projection(
        &mut tx,
    )
    .await
    .unwrap();

    let after =
        SqliteInventoryProjectionRepository
            ::list_all(
                &mut tx,
            )
            .await
            .unwrap();

    let after_view =
        materialize_hotel_inventory(
            after,
        );

    assert_eq!(
        before_view.len(),
        after_view.len(),
    );

    for (before, after)
        in before_view.iter()
            .zip(after_view.iter())
    {
        assert_eq!(
            before.date,
            after.date,
        );

        assert_eq!(
            before.reserved_rooms,
            after.reserved_rooms,
        );
    }
}