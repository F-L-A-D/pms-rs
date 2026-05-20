use chrono::Utc;

use serial_test::serial;

use uuid::Uuid;

use pms_rs::{
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::{
            reservation_booking::{ReservationBookingChannel, ReservationDailyStayDetail},
            room_daily_state::{RoomDailyOccupancyStatus, RoomDailyState},
        },
    },
    projection::{
        aggregate::access::fetch_inventory_aggregates_by_date::fetch_inventory_aggregates_by_date,
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
    repository::sqlite::operational::{
        reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
        reservation_repository::SqliteReservationRepository,
        room_daily_state_repository::SqliteRoomDailyStateRepository,
    },
};

use crate::common::{app::spawn_app, reservation::create_reservation, room::create_room};

#[tokio::test]
#[serial]
async fn should_refresh_inventory_aggregate_for_date() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let reservation = create_reservation(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    let aggregates = fetch_inventory_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 0);
    assert_eq!(aggregate.reservable_rooms, 1);
    assert_eq!(aggregate.confirmed_reservations, 1);
    assert_eq!(aggregate.pending_reservations, 0);
    assert_eq!(aggregate.available_rooms, 0);
    assert_eq!(aggregate.available_rooms_including_pending, 0);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_preserve_pending_and_out_of_order_inventory_inputs() {
    clear_trace();

    let app = spawn_app().await;

    let room = create_room(&app.app).await;
    let service_date = Utc::now().date_naive();

    let mut room_state = RoomDailyState::new(room.id, service_date);
    room_state.set_occupancy_status(RoomDailyOccupancyStatus::OutOfOrder);

    let pending_reservation = Reservation {
        id: Uuid::new_v4(),
        external_id: None,
        check_in: service_date,
        check_out: service_date.succ_opt().unwrap(),
        reservation_status: ReservationStatus::Pending,
        stay_status: Some(StayStatus::Confirmed),
        room_class: "standard".to_string(),
        room_id: None,
        booking_channel: ReservationBookingChannel::Direct,
        plan_code: None,
        version: 1,
        package_breakdowns: vec![],
        daily_stay_details: vec![],
        daily_revenue_allocations: vec![],
        participants: vec![],
        created_at: Utc::now(),
    };

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &room_state)
        .await
        .unwrap();

    SqliteReservationRepository::save(&mut tx, &pending_reservation)
        .await
        .unwrap();

    SqliteReservationDailyStayDetailRepository::save(
        &mut tx,
        &ReservationDailyStayDetail {
            reservation_id: pending_reservation.id,
            service_date,
            room_class: "standard".to_string(),
            plan_code: None,
            adult_count: 1,
            child_count: 0,
            sleep_sharing_child_count: 0,
            sleep_sharing_children: vec![],
        },
    )
    .await
    .unwrap();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::InventoryAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::InventoryDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let aggregates = fetch_inventory_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 1);
    assert_eq!(aggregate.reservable_rooms, 0);
    assert_eq!(aggregate.confirmed_reservations, 0);
    assert_eq!(aggregate.pending_reservations, 1);
    assert_eq!(aggregate.available_rooms, 0);
    assert_eq!(aggregate.available_rooms_including_pending, -1);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_execute_inventory_refresh_without_downstream_propagation() {
    clear_trace();

    let app = spawn_app().await;
    let reservation = create_reservation(&app.app).await;
    let mut tx = app.db.begin_tx().await;

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::InventoryAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::InventoryDate {
                date: reservation.check_in.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    assert_eq!(execution_trace(), vec![ProjectionNode::InventoryAggregate]);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_rebuild_inventory_aggregate_equivalent_to_refresh() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let reservation = create_reservation(&app.app).await;

    let mut tx = app.db.begin_tx().await;

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::InventoryAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::InventoryDate {
                date: reservation.check_in.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let refreshed = fetch_inventory_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::InventoryAggregate)
        .await
        .unwrap();

    let rebuilt = fetch_inventory_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    assert_eq!(refreshed, rebuilt);

    let _ = tx.rollback().await;
}
