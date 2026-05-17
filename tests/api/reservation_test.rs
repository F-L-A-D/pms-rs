use axum::http::StatusCode;

use chrono::{Duration, Utc};

use pms_rs::{
    api::dto::reservation::ReservationResponse,
    domain::{
        reservation_guest_relation::ReservationGuestRelationType,
        semantic::{
            reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
            reservation_transition::ReservationTransitionType,
        },
    },
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
            reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
            reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
        },
    },
};

use crate::common::{
    app::spawn_app,
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{get, patch_json, post_json, response_json},
    guest::create_guest,
    reservation::create_reservation,
};

#[tokio::test]
async fn should_roundtrip_reservation() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let response = get(&app.app, &format!("/reservations/{}", created.id,)).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let retrieved: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(retrieved.id, created.id,);

    assert_eq!(retrieved.room_class, created.room_class,);

    assert_eq!(retrieved.check_in, created.check_in,);

    assert_eq!(retrieved.check_out, created.check_out,);

    assert_eq!(retrieved.participants.len(), 1,);

    assert_eq!(
        retrieved.participants[0].guest_id,
        created.participants[0].guest_id,
    );
}

#[tokio::test]
async fn should_reject_invalid_stay_range() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let participant = ReservationParticipantBuilder::new(guest.id).build();

    let today = Utc::now().date_naive();

    let request = ReservationBuilder::new()
        .with_participant(participant)
        .with_check_in(today + Duration::days(2))
        .with_check_out(today)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_empty_participants() {
    let app = spawn_app().await;

    let request = ReservationBuilder::new().build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_multiple_primary_participants() {
    let app = spawn_app().await;

    let guest1 = create_guest(&app.app).await;

    let guest2 = create_guest(&app.app).await;

    let participant1 = ReservationParticipantBuilder::new(guest1.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let participant2 = ReservationParticipantBuilder::new(guest2.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let request = ReservationBuilder::new()
        .with_participant(participant1)
        .with_participant(participant2)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_reject_duplicate_participant_guest_ids() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;

    let participant1 = ReservationParticipantBuilder::new(guest.id)
        .with_relation_type(ReservationGuestRelationType::Primary)
        .build();

    let participant2 = ReservationParticipantBuilder::new(guest.id)
        .with_relation_type(ReservationGuestRelationType::Accompany)
        .build();

    let request = ReservationBuilder::new()
        .with_participant(participant1)
        .with_participant(participant2)
        .build();

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn should_create_reservation_with_booking_channel_and_package_breakdowns() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "external_id": "RES-PACKAGE-001",
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(1)).to_string(),
        "room_class": "standard",
        "booking_channel": "ota",
        "plan_code": "BB",
        "package_breakdowns": [
            {
                "package_code": "ROOM",
                "revenue_category": "room",
                "amount": "120.00"
            },
            {
                "package_code": "BREAKFAST",
                "revenue_category": "food_and_beverage",
                "amount": "30.00"
            },
            {
                "package_code": "TAX",
                "revenue_category": "tax",
                "amount": "15.00"
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(created.booking_channel, ReservationBookingChannel::Ota);
    assert_eq!(created.plan_code.as_deref(), Some("BB"));
    assert_eq!(created.package_breakdowns.len(), 3);
    assert!(created.package_breakdowns.iter().any(|breakdown| {
        breakdown.package_code == "ROOM"
            && breakdown.revenue_category == ReservationRevenueCategory::Room
            && breakdown.amount == rust_decimal::Decimal::new(12000, 2)
    }));

    let mut tx = app.db.begin_tx().await;

    let breakdowns =
        SqliteReservationPackageBreakdownRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(breakdowns.len(), 3);
    assert!(breakdowns.iter().any(|breakdown| {
        breakdown.package_code == "BREAKFAST"
            && breakdown.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && breakdown.amount == rust_decimal::Decimal::new(3000, 2)
    }));
}

#[tokio::test]
async fn should_create_reservation_with_daily_stay_details_and_daily_revenue_allocations() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "external_id": "RES-DAILY-001",
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
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let created: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(created.daily_details.len(), 2);
    assert!(created.daily_details.iter().any(|detail| {
        detail.service_date == today
            && detail.room_class == "standard"
            && detail.plan_code.as_deref() == Some("BB")
            && detail.adult_count == 1
            && detail.child_count == 0
    }));
    assert!(created.daily_details.iter().any(|detail| {
        detail.service_date == today + Duration::days(1)
            && detail.room_class == "deluxe"
            && detail.plan_code.as_deref() == Some("HB")
            && detail.adult_count == 2
            && detail.child_count == 1
    }));
    assert_eq!(created.daily_revenue_allocations.len(), 5);

    let mut tx = app.db.begin_tx().await;

    let details =
        SqliteReservationDailyStayDetailRepository::list_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();
    let allocations = SqliteReservationDailyRevenueAllocationRepository::list_by_reservation_id(
        &mut tx, created.id,
    )
    .await
    .unwrap();

    let _ = tx.rollback().await;

    assert_eq!(details.len(), 2);
    assert_eq!(allocations.len(), 5);
    assert!(allocations.iter().any(|allocation| {
        allocation.service_date == today + Duration::days(1)
            && allocation.package_code == "DINNER"
            && allocation.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && allocation.amount == rust_decimal::Decimal::new(4000, 2)
    }));
}

#[tokio::test]
async fn should_reject_daily_details_that_do_not_cover_every_reservation_night() {
    let app = spawn_app().await;

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let today = Utc::now().date_naive();

    let request = serde_json::json!({
        "check_in": today.to_string(),
        "check_out": (today + Duration::days(2)).to_string(),
        "room_class": "standard",
        "daily_details": [
            {
                "service_date": today.to_string(),
                "room_class": "standard",
                "adult_count": 1,
                "child_count": 0
            }
        ],
        "participants": [
            {
                "guest_id": participant.guest_id.to_string(),
                "relation_type": participant.relation_type
            }
        ]
    });

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_record_semantic_reservation_transitions_on_modify() {
    let app = spawn_app().await;

    let created = create_reservation(&app.app).await;

    let next_check_out = created.check_out + Duration::days(1);

    let request = serde_json::json!({
        "check_out": next_check_out.to_string(),
        "room_class": "deluxe"
    });

    let response = patch_json(&app.app, &format!("/reservations/{}", created.id), &request).await;

    assert_eq!(response.status(), StatusCode::OK,);

    let mut tx = app.db.begin_tx().await;

    let transitions =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, created.id)
            .await
            .unwrap();

    let _ = tx.rollback().await;

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::CheckOutChanged
            && transition.field_name == "check_out"
            && transition.before_value == created.check_out.to_string()
            && transition.after_value == next_check_out.to_string()
    }));

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::ReservationExtended
            && transition.field_name == "check_out"
            && transition.before_value == created.check_out.to_string()
            && transition.after_value == next_check_out.to_string()
    }));

    assert!(transitions.iter().any(|transition| {
        transition.transition_type == ReservationTransitionType::RoomClassChanged
            && transition.field_name == "room_class"
            && transition.before_value == created.room_class
            && transition.after_value == "deluxe"
    }));
}
