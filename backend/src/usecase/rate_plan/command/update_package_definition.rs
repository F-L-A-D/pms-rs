use crate::{
    db::connection::Db,
    domain::entity::package_definition::PackageDefinition,
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::rate_plan::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(db: &Db, package: PackageDefinition) -> AppResult<PackageDefinition> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqlitePackageDefinitionRepository::find_by_package_code(&mut tx, &package.package_code)
            .await?
            .ok_or_else(|| not_found("package not found"))?;

        if package.display_name.trim().is_empty() {
            return Err(validation("package display name must not be empty"));
        }

        if package.department_code.trim().is_empty() {
            return Err(validation("department code must not be empty"));
        }

        if package.account_code.trim().is_empty() {
            return Err(validation("account code must not be empty"));
        }

        SqlitePackageDefinitionRepository::save(&mut tx, &package).await?;

        Ok(package)
    }
    .await;

    match result {
        Ok(package) => {
            tx.commit().await.map_err(infra)?;

            Ok(package)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
