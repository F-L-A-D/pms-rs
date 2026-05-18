use crate::{
    db::connection::Db,
    domain::entity::package_definition::PackageDefinition,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(
    db: &Db,
    package_code: String,
    is_active: bool,
) -> AppResult<PackageDefinition> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut package =
            SqlitePackageDefinitionRepository::find_by_package_code(&mut tx, &package_code)
                .await?
                .ok_or_else(|| not_found("package not found"))?;

        package.is_active = is_active;

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
