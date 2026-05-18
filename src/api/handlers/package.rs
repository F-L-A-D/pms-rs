use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    api::{
        dto::package::{
            AssignPackageToPlanRequest, CreatePackageDefinitionRequest, CreateRatePlanRequest,
            PackageDefinitionResponse, RatePlanPackageResponse, RatePlanResponse,
            UpdateActivationRequest, UpdatePackageDefinitionRequest,
        },
        error::{map_app_error, ApiError},
        state::AppState,
    },
    domain::entity::package_definition::{PackageDefinition, RatePlanDefinition, RatePlanPackage},
    error::app_error::{infra, not_found},
    repository::sqlite::operational::package_definition_repository::SqlitePackageDefinitionRepository,
    usecase::rate_plan::command::{
        assign_package_to_plan, define_package, define_rate_plan, update_package_activation,
        update_package_definition, update_rate_plan_activation,
    },
};

pub async fn create_rate_plan_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateRatePlanRequest>,
) -> Result<(StatusCode, Json<RatePlanResponse>), ApiError> {
    let rate_plan = define_rate_plan::execute(
        &state.db,
        RatePlanDefinition {
            plan_code: req.plan_code,
            display_name: req.display_name,
            is_active: true,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::CREATED, Json(rate_plan.into())))
}

pub async fn get_rate_plan_handler(
    State(state): State<AppState>,
    Path(plan_code): Path<String>,
) -> Result<Json<RatePlanResponse>, ApiError> {
    let mut tx = state.db.begin_tx().await;

    let rate_plan = SqlitePackageDefinitionRepository::find_rate_plan_by_code(&mut tx, &plan_code)
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| map_app_error(not_found("rate plan not found")))?;

    let _ = tx.rollback().await.map_err(infra);

    Ok(Json(rate_plan.into()))
}

pub async fn create_package_definition_handler(
    State(state): State<AppState>,
    Json(req): Json<CreatePackageDefinitionRequest>,
) -> Result<(StatusCode, Json<PackageDefinitionResponse>), ApiError> {
    let package = define_package::execute(
        &state.db,
        PackageDefinition {
            package_code: req.package_code,
            display_name: req.display_name,
            revenue_category: req.revenue_category,
            department_code: req.department_code,
            account_code: req.account_code,
            is_active: true,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok((StatusCode::CREATED, Json(package.into())))
}

pub async fn update_package_definition_handler(
    State(state): State<AppState>,
    Path(package_code): Path<String>,
    Json(req): Json<UpdatePackageDefinitionRequest>,
) -> Result<Json<PackageDefinitionResponse>, ApiError> {
    let existing = {
        let mut tx = state.db.begin_tx().await;
        let package =
            SqlitePackageDefinitionRepository::find_by_package_code(&mut tx, &package_code)
                .await
                .map_err(map_app_error)?
                .ok_or_else(|| map_app_error(not_found("package not found")))?;
        let _ = tx.rollback().await.map_err(infra);
        package
    };

    let package = update_package_definition::execute(
        &state.db,
        PackageDefinition {
            package_code,
            display_name: req.display_name,
            revenue_category: req.revenue_category,
            department_code: req.department_code,
            account_code: req.account_code,
            is_active: existing.is_active,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(package.into()))
}

pub async fn update_package_activation_handler(
    State(state): State<AppState>,
    Path(package_code): Path<String>,
    Json(req): Json<UpdateActivationRequest>,
) -> Result<Json<PackageDefinitionResponse>, ApiError> {
    let package = update_package_activation::execute(&state.db, package_code, req.is_active)
        .await
        .map_err(map_app_error)?;

    Ok(Json(package.into()))
}

pub async fn get_package_definition_handler(
    State(state): State<AppState>,
    Path(package_code): Path<String>,
) -> Result<Json<PackageDefinitionResponse>, ApiError> {
    let mut tx = state.db.begin_tx().await;

    let package = SqlitePackageDefinitionRepository::find_by_package_code(&mut tx, &package_code)
        .await
        .map_err(map_app_error)?
        .ok_or_else(|| map_app_error(not_found("package not found")))?;

    let _ = tx.rollback().await.map_err(infra);

    Ok(Json(package.into()))
}

pub async fn assign_package_to_plan_handler(
    State(state): State<AppState>,
    Path(plan_code): Path<String>,
    Json(req): Json<AssignPackageToPlanRequest>,
) -> Result<Json<RatePlanPackageResponse>, ApiError> {
    let assignment = assign_package_to_plan::execute(
        &state.db,
        RatePlanPackage {
            plan_code,
            package_code: req.package_code,
            sort_order: req.sort_order,
        },
    )
    .await
    .map_err(map_app_error)?;

    Ok(Json(assignment.into()))
}

pub async fn update_rate_plan_activation_handler(
    State(state): State<AppState>,
    Path(plan_code): Path<String>,
    Json(req): Json<UpdateActivationRequest>,
) -> Result<Json<RatePlanResponse>, ApiError> {
    let rate_plan = update_rate_plan_activation::execute(&state.db, plan_code, req.is_active)
        .await
        .map_err(map_app_error)?;

    Ok(Json(rate_plan.into()))
}

pub async fn list_plan_packages_handler(
    State(state): State<AppState>,
    Path(plan_code): Path<String>,
) -> Result<Json<Vec<RatePlanPackageResponse>>, ApiError> {
    let mut tx = state.db.begin_tx().await;

    let assignments = SqlitePackageDefinitionRepository::list_by_plan_code(&mut tx, &plan_code)
        .await
        .map_err(map_app_error)?;

    let _ = tx.rollback().await.map_err(infra);

    Ok(Json(
        assignments
            .into_iter()
            .map(RatePlanPackageResponse::from)
            .collect(),
    ))
}

impl From<PackageDefinition> for PackageDefinitionResponse {
    fn from(package: PackageDefinition) -> Self {
        Self {
            package_code: package.package_code,
            display_name: package.display_name,
            revenue_category: package.revenue_category,
            department_code: package.department_code,
            account_code: package.account_code,
            is_active: package.is_active,
        }
    }
}

impl From<RatePlanDefinition> for RatePlanResponse {
    fn from(rate_plan: RatePlanDefinition) -> Self {
        Self {
            plan_code: rate_plan.plan_code,
            display_name: rate_plan.display_name,
            is_active: rate_plan.is_active,
        }
    }
}

impl From<RatePlanPackage> for RatePlanPackageResponse {
    fn from(assignment: RatePlanPackage) -> Self {
        Self {
            plan_code: assignment.plan_code,
            package_code: assignment.package_code,
            sort_order: assignment.sort_order,
        }
    }
}
