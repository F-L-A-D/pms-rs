use chrono::Utc;

use serde_json::json;

use serial_test::serial;

use pms_rs::{
    domain::semantic::room_daily_state::{RoomDailyOccupancyStatus, RoomDailyState},
    projection::{
        aggregate::access::fetch_housekeeping_daily_workload_aggregates_by_date::fetch_housekeeping_daily_workload_aggregates_by_date,
        execution::execution_trace::{clear_trace, execution_trace},
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::{
            rebuild_projection_chain::rebuild_projection_chain,
            refresh_projection_chain::refresh_projection_chain,
        },
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::room_daily_state_repository::SqliteRoomDailyStateRepository,
};

use crate::common::{app::spawn_app, client::post_json, room::create_room};

#[tokio::test]
#[serial]
async fn should_refresh_housekeeping_daily_workload_from_housekeeping_command() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = Utc::now().date_naive();
    let body = json!({ "service_date": service_date.to_string() });

    let response = post_json(&app.app, &format!("/housekeeping/{}/dirty", room.id), &body).await;

    assert!(response.status().is_success());

    let mut tx = app.db.begin_tx().await;

    let aggregates = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_tracked_rooms, 1);
    assert_eq!(aggregate.dirty_rooms, 1);
    assert_eq!(aggregate.cleaning_rooms, 0);
    assert_eq!(aggregate.cleaned_rooms, 0);
    assert_eq!(aggregate.inspected_rooms, 0);
    assert_eq!(aggregate.vacant_rooms, 1);
    assert_eq!(aggregate.occupied_rooms, 0);
    assert_eq!(aggregate.out_of_order_rooms, 0);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_preserve_housekeeping_and_occupancy_counts() {
    clear_trace();

    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = Utc::now().date_naive();

    let mut room_state = RoomDailyState::new(room.id, service_date);
    room_state.mark_dirty();
    room_state.set_occupancy_status(RoomDailyOccupancyStatus::OutOfOrder);

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &room_state)
        .await
        .unwrap();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::HousekeepingDailyWorkloadAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::RoomDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let aggregates = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_tracked_rooms, 1);
    assert_eq!(aggregate.dirty_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 1);
    assert_eq!(aggregate.vacant_rooms, 0);
    assert_eq!(aggregate.occupied_rooms, 0);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_execute_housekeeping_workload_refresh_without_downstream_propagation() {
    clear_trace();

    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = Utc::now().date_naive();
    let mut tx = app.db.begin_tx().await;

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::HousekeepingDailyWorkloadAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::RoomDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    assert_eq!(
        execution_trace(),
        vec![ProjectionNode::HousekeepingDailyWorkloadAggregate]
    );

    let _ = tx.rollback().await;
    let _ = room;
}

#[tokio::test]
#[serial]
async fn should_rebuild_housekeeping_workload_equivalent_to_refresh() {
    let app = spawn_app().await;
    let room = create_room(&app.app).await;
    let service_date = Utc::now().date_naive();

    let mut room_state = RoomDailyState::new(room.id, service_date);
    room_state.mark_dirty();

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &room_state)
        .await
        .unwrap();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::HousekeepingDailyWorkloadAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::RoomDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let refreshed = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::HousekeepingDailyWorkloadAggregate)
        .await
        .unwrap();

    let rebuilt = fetch_housekeeping_daily_workload_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    assert_eq!(refreshed, rebuilt);

    let _ = tx.rollback().await;
}
