use axum::http::StatusCode;

use pms_rs::{
    api::dto::{
        package::{PackageDefinitionResponse, RatePlanPackageResponse, RatePlanResponse},
        reservation::ReservationResponse,
    },
    domain::semantic::reservation_booking::ReservationRevenueCategory,
};

use crate::common::{
    app::spawn_app,
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{get, post_json, response_json},
    guest::create_guest,
};

#[tokio::test]
async fn should_create_package_definition_and_assign_it_to_rate_plan() {
    let app = spawn_app().await;

    let create_response = post_json(
        &app.app,
        "/packages",
        &serde_json::json!({
            "package_code": "BREAKFAST",
            "display_name": "Breakfast",
            "revenue_category": "food_and_beverage",
            "department_code": "FNB",
            "account_code": "4100"
        }),
    )
    .await;

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let package: PackageDefinitionResponse =
        serde_json::from_value(response_json(create_response).await).unwrap();

    assert_eq!(package.package_code, "BREAKFAST");
    assert_eq!(
        package.revenue_category,
        ReservationRevenueCategory::FoodAndBeverage
    );
    assert_eq!(package.department_code, "FNB");
    assert_eq!(package.account_code, "4100");

    let rate_plan_response = post_json(
        &app.app,
        "/rate-plans",
        &serde_json::json!({
            "plan_code": "BB",
            "display_name": "Bed and Breakfast"
        }),
    )
    .await;

    assert_eq!(rate_plan_response.status(), StatusCode::CREATED);

    let rate_plan: RatePlanResponse =
        serde_json::from_value(response_json(rate_plan_response).await).unwrap();

    assert_eq!(rate_plan.plan_code, "BB");

    let assign_response = post_json(
        &app.app,
        "/rate-plans/BB/packages",
        &serde_json::json!({
            "package_code": "BREAKFAST",
            "sort_order": 10
        }),
    )
    .await;

    assert_eq!(assign_response.status(), StatusCode::OK);

    let assignment: RatePlanPackageResponse =
        serde_json::from_value(response_json(assign_response).await).unwrap();

    assert_eq!(assignment.plan_code, "BB");
    assert_eq!(assignment.package_code, "BREAKFAST");

    let list_response = get(&app.app, "/rate-plans/BB/packages").await;

    assert_eq!(list_response.status(), StatusCode::OK);

    let assignments: Vec<RatePlanPackageResponse> =
        serde_json::from_value(response_json(list_response).await).unwrap();

    assert_eq!(assignments.len(), 1);
    assert_eq!(assignments[0].package_code, "BREAKFAST");
}

#[tokio::test]
async fn should_reject_rate_plan_assignment_for_missing_package() {
    let app = spawn_app().await;

    let rate_plan_response = post_json(
        &app.app,
        "/rate-plans",
        &serde_json::json!({
            "plan_code": "BB",
            "display_name": "Bed and Breakfast"
        }),
    )
    .await;

    assert_eq!(rate_plan_response.status(), StatusCode::CREATED);

    let response = post_json(
        &app.app,
        "/rate-plans/BB/packages",
        &serde_json::json!({
            "package_code": "MISSING",
            "sort_order": 10
        }),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn should_resolve_reservation_package_breakdown_category_from_package_catalog() {
    let app = spawn_app().await;

    let package_response = post_json(
        &app.app,
        "/packages",
        &serde_json::json!({
            "package_code": "DINNER",
            "display_name": "Dinner",
            "revenue_category": "food_and_beverage",
            "department_code": "FNB",
            "account_code": "4200"
        }),
    )
    .await;

    assert_eq!(package_response.status(), StatusCode::CREATED);

    let guest = create_guest(&app.app).await;
    let participant = ReservationParticipantBuilder::new(guest.id).build();
    let mut request = serde_json::to_value(
        ReservationBuilder::new()
            .with_participant(participant)
            .build(),
    )
    .unwrap();

    request["plan_code"] = serde_json::json!("HB");
    request["package_breakdowns"] = serde_json::json!([
        {
            "package_code": "DINNER",
            "revenue_category": "other",
            "amount": "5000.00"
        }
    ]);

    let response = post_json(&app.app, "/reservations", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let reservation: ReservationResponse =
        serde_json::from_value(response_json(response).await).unwrap();

    assert_eq!(reservation.package_breakdowns.len(), 1);
    assert_eq!(
        reservation.package_breakdowns[0].revenue_category,
        ReservationRevenueCategory::FoodAndBeverage
    );
    assert_eq!(reservation.daily_revenue_allocations.len(), 1);
    assert_eq!(
        reservation.daily_revenue_allocations[0].revenue_category,
        ReservationRevenueCategory::FoodAndBeverage
    );
    assert_eq!(
        reservation.daily_revenue_allocations[0]
            .department_code
            .as_deref(),
        Some("FNB")
    );
    assert_eq!(
        reservation.daily_revenue_allocations[0]
            .account_code
            .as_deref(),
        Some("4200")
    );
}
