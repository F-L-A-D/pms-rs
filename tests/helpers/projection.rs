use uuid::Uuid;

use pms_rs::db::connection::Db;

use pms_rs::projection::materializer::
    guest_summary_materializer::
    materialize_guest_summary;

use pms_rs::projection::rebuild::
    guest_summary_rebuild::
    rebuild_guest_summary_projection;

use pms_rs::projection::crm::
    guest_summary::GuestSummaryProjection;

pub async fn materialize_guest_summary_projection(
    db: &Db,
    guest_id: Uuid,
) -> GuestSummaryProjection {

    let mut tx =
        db.begin_tx().await;

    let projection =
        materialize_guest_summary(
            &mut tx,
            guest_id,
        )
        .await
        .unwrap();

    tx.rollback()
        .await
        .unwrap();

    projection
}

pub async fn rebuild_guest_summary(
    db: &Db,
) {

    let mut tx =
        db.begin_tx().await;

    rebuild_guest_summary_projection(
        &mut tx,
    )
    .await
    .unwrap();

    tx.commit()
        .await
        .unwrap();
}