use crate::{
    db::connection::Db,
    domain::entity::package_definition::RatePlanPackage,
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    repository::sqlite::operational::rate_plan::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(db: &Db, assignment: RatePlanPackage) -> AppResult<RatePlanPackage> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if assignment.plan_code.trim().is_empty() {
            return Err(validation("plan code must not be empty"));
        }

        if assignment.package_code.trim().is_empty() {
            return Err(validation("package code must not be empty"));
        }

        if assignment.sort_order < 0 {
            return Err(validation("sort order must not be negative"));
        }

        let rate_plan = SqlitePackageDefinitionRepository::find_rate_plan_by_code(
            &mut tx,
            &assignment.plan_code,
        )
        .await?
        .ok_or_else(|| not_found("rate plan not found"))?;

        if !rate_plan.is_active {
            return Err(conflict("rate plan inactive"));
        }

        let package = SqlitePackageDefinitionRepository::find_by_package_code(
            &mut tx,
            &assignment.package_code,
        )
        .await?
        .ok_or_else(|| not_found("package not found"))?;

        if !package.is_active {
            return Err(conflict("package inactive"));
        }

        SqlitePackageDefinitionRepository::assign_to_plan(&mut tx, &assignment).await?;

        Ok(assignment)
    }
    .await;

    match result {
        Ok(assignment) => {
            tx.commit().await.map_err(infra)?;

            Ok(assignment)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
