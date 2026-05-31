use chrono::{Datelike, Duration, NaiveDate, Utc};

use rust_decimal::Decimal;

use serde_json::json;

use serial_test::serial;

use pms_rs::{
    api::dto::response::reservation::ReservationResponse,
    domain::semantic::room_daily_state::{RoomDailyOccupancyStatus, RoomDailyState},
    projection::{
        aggregate::access::{
            fetch_daily_hotel_kpi_aggregate_by_date::fetch_daily_hotel_kpi_aggregate_by_date,
            fetch_daily_room_class_kpi_aggregates_by_date::fetch_daily_room_class_kpi_aggregates_by_date,
            fetch_monthly_hotel_kpi_aggregate_by_month::fetch_monthly_hotel_kpi_aggregate_by_month,
            fetch_monthly_room_class_kpi_aggregates_by_month::fetch_monthly_room_class_kpi_aggregates_by_month,
        },
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
    repository::sqlite::operational::room::room_daily_state_repository::SqliteRoomDailyStateRepository,
};

use crate::common::{
    app::spawn_app,
    business_date::current_open_business_date,
    client::{post_json, response_json},
    guest::create_guest,
    room::create_room,
};

#[tokio::test]
#[serial]
async fn should_refresh_daily_room_class_kpi_from_reservation_package_breakdowns() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let check_in = current_open_business_date(&app.app).await;
    let reservation = create_reservation_with_breakdowns(
        &app.app,
        check_in,
        check_in + Duration::days(2),
        vec![
            ("room", "room", "240.00"),
            ("breakfast", "food_and_beverage", "40.00"),
            ("tax", "tax", "20.00"),
        ],
    )
    .await;

    let mut tx = app.db.begin_tx().await;

    let aggregates = fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 0);
    assert_eq!(aggregate.reservable_rooms, 1);
    assert_eq!(aggregate.sold_room_nights, 1);
    assert_eq!(aggregate.occupied_rooms, 0);
    assert_eq!(aggregate.room_revenue, Decimal::new(12000, 2));
    assert_eq!(aggregate.food_and_beverage_revenue, Decimal::new(2000, 2));
    assert_eq!(aggregate.tax_amount, Decimal::new(1000, 2));
    assert_eq!(aggregate.total_revenue, Decimal::new(15000, 2));
    assert_eq!(aggregate.occupancy_rate, Decimal::new(1, 0));
    assert_eq!(aggregate.adr, Decimal::new(12000, 2));
    assert_eq!(aggregate.revpar, Decimal::new(12000, 2));

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_refresh_daily_hotel_kpi_from_reservation_package_breakdowns() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let check_in = current_open_business_date(&app.app).await;
    let reservation = create_reservation_with_breakdowns(
        &app.app,
        check_in,
        check_in + Duration::days(2),
        vec![
            ("room", "room", "240.00"),
            ("breakfast", "food_and_beverage", "40.00"),
            ("tax", "tax", "20.00"),
        ],
    )
    .await;

    let mut tx = app.db.begin_tx().await;

    let aggregate = fetch_daily_hotel_kpi_aggregate_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(aggregate.total_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 0);
    assert_eq!(aggregate.reservable_rooms, 1);
    assert_eq!(aggregate.sold_room_nights, 1);
    assert_eq!(aggregate.room_revenue, Decimal::new(12000, 2));
    assert_eq!(aggregate.food_and_beverage_revenue, Decimal::new(2000, 2));
    assert_eq!(aggregate.tax_amount, Decimal::new(1000, 2));
    assert_eq!(aggregate.total_revenue, Decimal::new(15000, 2));
    assert_eq!(aggregate.occupancy_rate, Decimal::new(1, 0));
    assert_eq!(aggregate.adr, Decimal::new(12000, 2));
    assert_eq!(aggregate.revpar, Decimal::new(12000, 2));

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_refresh_monthly_room_class_and_hotel_kpis() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let today = current_open_business_date(&app.app).await;
    let check_in = NaiveDate::from_ymd_opt(today.year(), today.month(), 2).unwrap();
    let check_out = check_in + Duration::days(2);
    let year_month = check_in.format("%Y-%m").to_string();

    create_reservation_with_breakdowns(
        &app.app,
        check_in,
        check_out,
        vec![
            ("room", "room", "240.00"),
            ("breakfast", "food_and_beverage", "40.00"),
            ("tax", "tax", "20.00"),
        ],
    )
    .await;

    let mut tx = app.db.begin_tx().await;

    let room_class_aggregates =
        fetch_monthly_room_class_kpi_aggregates_by_month(&mut tx, &year_month)
            .await
            .unwrap();

    let room_class_aggregate = room_class_aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    let days = days_in_month(check_in);

    assert_eq!(room_class_aggregate.total_room_nights, days);
    assert_eq!(room_class_aggregate.reservable_room_nights, days);
    assert_eq!(room_class_aggregate.sold_room_nights, 2);
    assert_eq!(room_class_aggregate.room_revenue, Decimal::new(24000, 2));
    assert_eq!(
        room_class_aggregate.food_and_beverage_revenue,
        Decimal::new(4000, 2)
    );
    assert_eq!(room_class_aggregate.tax_amount, Decimal::new(2000, 2));
    assert_eq!(room_class_aggregate.total_revenue, Decimal::new(30000, 2));
    assert_eq!(
        room_class_aggregate.occupancy_rate,
        Decimal::from(2) / Decimal::from(days)
    );
    assert_eq!(room_class_aggregate.adr, Decimal::new(12000, 2));
    assert_eq!(
        room_class_aggregate.revpar,
        Decimal::new(24000, 2) / Decimal::from(days)
    );

    let hotel_aggregate = fetch_monthly_hotel_kpi_aggregate_by_month(&mut tx, &year_month)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(hotel_aggregate.total_room_nights, days);
    assert_eq!(hotel_aggregate.reservable_room_nights, days);
    assert_eq!(hotel_aggregate.sold_room_nights, 2);
    assert_eq!(hotel_aggregate.room_revenue, Decimal::new(24000, 2));
    assert_eq!(hotel_aggregate.total_revenue, Decimal::new(30000, 2));
    assert_eq!(
        hotel_aggregate.occupancy_rate,
        Decimal::from(2) / Decimal::from(days)
    );

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_project_daily_kpis_from_nightly_room_class_and_revenue_allocations() {
    let app = spawn_app().await;

    create_room(&app.app).await;
    create_room_with_class(&app.app, "room_deluxe_daily_kpi", "deluxe").await;

    let guest = create_guest(&app.app).await;
    let today = current_open_business_date(&app.app).await;

    let body = json!({
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(2)).to_string(),
        "room_class": "standard",
        "booking_channel": "direct",
        "plan_code": "BASE",
        "daily_details": [
            {
                "service_date": today.to_string(),
                "room_class": "standard",
                "plan_code": "BB",
                "adult_count": 1,
                "child_count": 0,
                "package_breakdowns": [
                    {
                        "package_code": "ROOM",
                        "revenue_category": "room",
                        "amount": "100.00"
                    },
                    {
                        "package_code": "TAX",
                        "revenue_category": "tax",
                        "amount": "10.00"
                    }
                ]
            },
            {
                "service_date": (today + Duration::days(1)).to_string(),
                "room_class": "deluxe",
                "plan_code": "HB",
                "adult_count": 2,
                "child_count": 1,
                "package_breakdowns": [
                    {
                        "package_code": "ROOM",
                        "revenue_category": "room",
                        "amount": "180.00"
                    },
                    {
                        "package_code": "DINNER",
                        "revenue_category": "food_and_beverage",
                        "amount": "40.00"
                    },
                    {
                        "package_code": "TAX",
                        "revenue_category": "tax",
                        "amount": "20.00"
                    }
                ]
            }
        ],
        "participants": [
            {
                "guest_id": guest.id.to_string(),
                "relation_type": "primary"
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &body).await;

    assert!(response.status().is_success());

    let mut tx = app.db.begin_tx().await;

    let first_day = fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, today)
        .await
        .unwrap();
    let first_day_standard = first_day
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(first_day_standard.sold_room_nights, 1);
    assert_eq!(first_day_standard.room_revenue, Decimal::new(10000, 2));
    assert_eq!(first_day_standard.tax_amount, Decimal::new(1000, 2));

    let second_day =
        fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, today + Duration::days(1))
            .await
            .unwrap();
    let second_day_deluxe = second_day
        .iter()
        .find(|aggregate| aggregate.room_class == "deluxe")
        .unwrap();

    assert_eq!(second_day_deluxe.sold_room_nights, 1);
    assert_eq!(second_day_deluxe.room_revenue, Decimal::new(18000, 2));
    assert_eq!(
        second_day_deluxe.food_and_beverage_revenue,
        Decimal::new(4000, 2)
    );
    assert_eq!(second_day_deluxe.tax_amount, Decimal::new(2000, 2));

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_keep_zero_kpi_rates_when_all_rooms_are_out_of_order() {
    clear_trace();

    let app = spawn_app().await;

    let room = create_room(&app.app).await;
    let service_date = current_open_business_date(&app.app).await;

    let mut room_state = RoomDailyState::new(room.id, service_date);
    room_state.set_occupancy_status(RoomDailyOccupancyStatus::OutOfOrder);

    let mut tx = app.db.begin_tx().await;

    SqliteRoomDailyStateRepository::save(&mut tx, &room_state)
        .await
        .unwrap();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let aggregates = fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, service_date)
        .await
        .unwrap();

    let aggregate = aggregates
        .iter()
        .find(|aggregate| aggregate.room_class == "standard")
        .unwrap();

    assert_eq!(aggregate.total_rooms, 1);
    assert_eq!(aggregate.out_of_order_rooms, 1);
    assert_eq!(aggregate.reservable_rooms, 0);
    assert_eq!(aggregate.occupancy_rate, Decimal::ZERO);
    assert_eq!(aggregate.adr, Decimal::ZERO);
    assert_eq!(aggregate.revpar, Decimal::ZERO);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_execute_daily_room_class_kpi_refresh_without_downstream_propagation() {
    clear_trace();

    let app = spawn_app().await;
    create_room(&app.app).await;
    let service_date = current_open_business_date(&app.app).await;
    let mut tx = app.db.begin_tx().await;

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    assert_eq!(
        execution_trace(),
        vec![ProjectionNode::DailyRoomClassKpiAggregate]
    );

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_execute_hotel_and_monthly_kpi_refreshes_without_downstream_propagation() {
    clear_trace();

    let app = spawn_app().await;
    create_room(&app.app).await;
    let service_date = current_open_business_date(&app.app).await;
    let year_month = service_date.format("%Y-%m").to_string();
    let mut tx = app.db.begin_tx().await;

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyHotelKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    assert_eq!(
        execution_trace(),
        vec![ProjectionNode::DailyHotelKpiAggregate]
    );

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::MonthlyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiMonth {
                year_month: year_month.clone(),
            },
        ),
    )
    .await
    .unwrap();

    assert_eq!(
        execution_trace(),
        vec![ProjectionNode::MonthlyRoomClassKpiAggregate]
    );

    clear_trace();

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::MonthlyHotelKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiMonth { year_month },
        ),
    )
    .await
    .unwrap();

    assert_eq!(
        execution_trace(),
        vec![ProjectionNode::MonthlyHotelKpiAggregate]
    );

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_rebuild_daily_room_class_kpi_equivalent_to_refresh() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let check_in = current_open_business_date(&app.app).await;
    let reservation = create_reservation_with_breakdowns(
        &app.app,
        check_in,
        check_in + Duration::days(1),
        vec![("room", "room", "120.00"), ("tax", "tax", "12.00")],
    )
    .await;

    let mut tx = app.db.begin_tx().await;

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: reservation.check_in.to_string(),
            },
        ),
    )
    .await
    .unwrap();

    let refreshed = fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::DailyRoomClassKpiAggregate)
        .await
        .unwrap();

    let rebuilt = fetch_daily_room_class_kpi_aggregates_by_date(&mut tx, reservation.check_in)
        .await
        .unwrap();

    assert_eq!(refreshed, rebuilt);

    let _ = tx.rollback().await;
}

#[tokio::test]
#[serial]
async fn should_rebuild_monthly_hotel_kpi_equivalent_to_refresh() {
    let app = spawn_app().await;

    create_room(&app.app).await;

    let today = current_open_business_date(&app.app).await;
    let check_in = NaiveDate::from_ymd_opt(today.year(), today.month(), 2).unwrap();
    let year_month = check_in.format("%Y-%m").to_string();

    create_reservation_with_breakdowns(
        &app.app,
        check_in,
        check_in + Duration::days(1),
        vec![("room", "room", "120.00"), ("tax", "tax", "12.00")],
    )
    .await;

    let mut tx = app.db.begin_tx().await;

    refresh_projection_chain(
        &mut tx,
        ProjectionInvalidation::new(
            ProjectionNode::MonthlyHotelKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiMonth {
                year_month: year_month.clone(),
            },
        ),
    )
    .await
    .unwrap();

    let refreshed = fetch_monthly_hotel_kpi_aggregate_by_month(&mut tx, &year_month)
        .await
        .unwrap();

    rebuild_projection_chain(&mut tx, ProjectionNode::MonthlyHotelKpiAggregate)
        .await
        .unwrap();

    let rebuilt = fetch_monthly_hotel_kpi_aggregate_by_month(&mut tx, &year_month)
        .await
        .unwrap();

    assert_eq!(refreshed, rebuilt);

    let _ = tx.rollback().await;
}

async fn create_reservation_with_breakdowns(
    app: &axum::Router,
    check_in: NaiveDate,
    check_out: NaiveDate,
    breakdowns: Vec<(&str, &str, &str)>,
) -> ReservationResponse {
    let guest = create_guest(app).await;

    let body = json!({
        "check_in": check_in.to_string(),
        "check_out": check_out.to_string(),
        "room_class": "standard",
        "booking_channel": "ota",
        "plan_code": "bb",
        "package_breakdowns": breakdowns
            .into_iter()
            .map(|(package_code, revenue_category, amount)| {
                json!({
                    "package_code": package_code,
                    "revenue_category": revenue_category,
                    "amount": amount
                })
            })
            .collect::<Vec<_>>(),
        "participants": [
            {
                "guest_id": guest.id.to_string(),
                "relation_type": "primary"
            }
        ]
    });

    let response = post_json(app, "/reservations", &body).await;

    assert!(response.status().is_success());

    let body = response_json(response).await;

    serde_json::from_value::<ReservationResponse>(body).unwrap()
}

fn days_in_month(date: NaiveDate) -> i64 {
    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap()
    };

    i64::from(next_month.pred_opt().unwrap().day())
}

async fn create_room_with_class(app: &axum::Router, room_no: &str, room_class: &str) {
    let body = json!({
        "room_no": room_no,
        "room_class": room_class
    });

    let response = post_json(app, "/rooms", &body).await;

    assert!(response.status().is_success());
}
