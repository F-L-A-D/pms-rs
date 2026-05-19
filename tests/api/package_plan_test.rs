use axum::http::StatusCode;

use pms_rs::{
    api::dto::{
        package::{PackageDefinitionResponse, RatePlanPackageResponse, RatePlanResponse},
        reservation::ReservationResponse,
        revenue_summary::RevenueSummaryLineResponse,
    },
    domain::semantic::reservation_booking::ReservationRevenueCategory,
};

use crate::common::{
    app::spawn_app,
    builders::{ReservationBuilder, ReservationParticipantBuilder},
    client::{get, patch_json, post_json, response_json},
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
async fn should_update_package_and_rate_plan_activation() {
    let app = spawn_app().await;

    let package_response = post_json(
        &app.app,
        "/packages",
        &serde_json::json!({
            "package_code": "SPA",
            "display_name": "Spa",
            "revenue_category": "other",
            "department_code": "SPA",
            "account_code": "4300"
        }),
    )
    .await;

    assert_eq!(package_response.status(), StatusCode::CREATED);

    let update_response = patch_json(
        &app.app,
        "/packages/SPA",
        &serde_json::json!({
            "display_name": "Spa Package",
            "revenue_category": "other",
            "department_code": "WELLNESS",
            "account_code": "4310"
        }),
    )
    .await;

    assert_eq!(update_response.status(), StatusCode::OK);

    let package: PackageDefinitionResponse =
        serde_json::from_value(response_json(update_response).await).unwrap();

    assert_eq!(package.display_name, "Spa Package");
    assert_eq!(package.department_code, "WELLNESS");
    assert!(package.is_active);

    let deactivate_package = patch_json(
        &app.app,
        "/packages/SPA/activation",
        &serde_json::json!({ "is_active": false }),
    )
    .await;

    assert_eq!(deactivate_package.status(), StatusCode::OK);

    let package: PackageDefinitionResponse =
        serde_json::from_value(response_json(deactivate_package).await).unwrap();

    assert!(!package.is_active);

    let rate_plan_response = post_json(
        &app.app,
        "/rate-plans",
        &serde_json::json!({
            "plan_code": "WELL",
            "display_name": "Wellness"
        }),
    )
    .await;

    assert_eq!(rate_plan_response.status(), StatusCode::CREATED);

    let deactivate_rate_plan = patch_json(
        &app.app,
        "/rate-plans/WELL/activation",
        &serde_json::json!({ "is_active": false }),
    )
    .await;

    assert_eq!(deactivate_rate_plan.status(), StatusCode::OK);

    let rate_plan: RatePlanResponse =
        serde_json::from_value(response_json(deactivate_rate_plan).await).unwrap();

    assert!(!rate_plan.is_active);
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

    let summary_response = get(
        &app.app,
        &format!("/revenue-summary/daily?date={}", reservation.check_in),
    )
    .await;

    assert_eq!(summary_response.status(), StatusCode::OK);

    let summary: Vec<RevenueSummaryLineResponse> =
        serde_json::from_value(response_json(summary_response).await).unwrap();

    assert!(summary.iter().any(|line| {
        line.revenue_category == ReservationRevenueCategory::FoodAndBeverage
            && line.department_code.as_deref() == Some("FNB")
            && line.account_code.as_deref() == Some("4200")
            && line.amount.to_string() == "5000"
    }));
}
