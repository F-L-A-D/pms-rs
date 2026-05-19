use crate::{
    db::connection::Db,
    domain::entity::package_definition::RatePlanDefinition,
    error::app_error::{infra, validation, AppResult},
    repository::sqlite::operational::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(db: &Db, rate_plan: RatePlanDefinition) -> AppResult<RatePlanDefinition> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if rate_plan.plan_code.trim().is_empty() {
            return Err(validation("plan code must not be empty"));
        }

        if rate_plan.display_name.trim().is_empty() {
            return Err(validation("plan display name must not be empty"));
        }

        SqlitePackageDefinitionRepository::save_rate_plan(&mut tx, &rate_plan).await?;

        Ok(rate_plan)
    }
    .await;

    match result {
        Ok(rate_plan) => {
            tx.commit().await.map_err(infra)?;

            Ok(rate_plan)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
