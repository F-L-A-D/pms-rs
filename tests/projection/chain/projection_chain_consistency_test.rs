use crate::helpers::{
    app::test_app,
    guest::create_guest,
    reservation::create_reservation,
};

use pms_rs::projection::{
    orchestrator::
        rebuild_projection_chain::
        rebuild_projection_chain,

    topology::
        projection_node::
        ProjectionNode,
};

use pms_rs::repository::sqlite::projection::{
    hotel_inventory_projection_repository::
        SqliteHotelInventoryProjectionRepository,

    inventory_projection_repository::
        SqliteInventoryProjectionRepository,
};

#[tokio::test]
async fn should_match_incremental_refresh_and_full_chain_rebuild()
{
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

    //
    // incremental refresh result
    //

    let incremental =
        SqliteHotelInventoryProjectionRepository
            ::find_by_date(
                &mut tx,
                "2026-05-10",
            )
            .await
            .unwrap();

    //
    // delete projections
    //

    SqliteInventoryProjectionRepository
        ::delete_all(&mut tx)
        .await
        .unwrap();

    SqliteHotelInventoryProjectionRepository
        ::delete_all(&mut tx)
        .await
        .unwrap();

    //
    // full rebuild
    //

    rebuild_projection_chain(
        &mut tx,
        ProjectionNode::Inventory,
    )
    .await
    .unwrap();

    //
    // rebuilt result
    //

    let rebuilt =
        SqliteHotelInventoryProjectionRepository
            ::find_by_date(
                &mut tx,
                "2026-05-10",
            )
            .await
            .unwrap();

    //
    // equivalence
    //

    assert_eq!(
        incremental,
        rebuilt,
    );
}