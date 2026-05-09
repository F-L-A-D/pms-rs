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

use pms_rs::repository::sqlite::projection::
    hotel_inventory_projection_repository::
        SqliteHotelInventoryProjectionRepository;

#[tokio::test]
async fn should_rebuild_projection_chain()
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

    rebuild_projection_chain(
        &mut tx,
        ProjectionNode::Inventory,
    )
    .await
    .unwrap();

    let projection =
        SqliteHotelInventoryProjectionRepository
            ::find_by_date(
                &mut tx,
                "2026-05-10",
            )
            .await
            .unwrap();

    assert!(
        projection.is_some()
    );
}