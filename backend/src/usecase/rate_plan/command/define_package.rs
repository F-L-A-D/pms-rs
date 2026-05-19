use crate::{
    db::connection::Db,
    domain::entity::package_definition::PackageDefinition,
    error::app_error::{infra, validation, AppResult},
    repository::sqlite::operational::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(db: &Db, package: PackageDefinition) -> AppResult<PackageDefinition> {
    let mut tx = db.begin_tx().await;

    let result = async {
        validate_package(&package)?;

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

fn validate_package(package: &PackageDefinition) -> AppResult<()> {
    if package.package_code.trim().is_empty() {
        return Err(validation("package code must not be empty"));
    }

    if package.display_name.trim().is_empty() {
        return Err(validation("package display name must not be empty"));
    }

    if package.department_code.trim().is_empty() {
        return Err(validation("department code must not be empty"));
    }

    if package.account_code.trim().is_empty() {
        return Err(validation("account code must not be empty"));
    }

    Ok(())
}
