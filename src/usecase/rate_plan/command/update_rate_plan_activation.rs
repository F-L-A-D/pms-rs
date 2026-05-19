use crate::{
    db::connection::Db,
    domain::entity::package_definition::RatePlanDefinition,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::package_definition_repository::SqlitePackageDefinitionRepository,
};

pub async fn execute(db: &Db, plan_code: String, is_active: bool) -> AppResult<RatePlanDefinition> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut rate_plan =
            SqlitePackageDefinitionRepository::find_rate_plan_by_code(&mut tx, &plan_code)
                .await?
                .ok_or_else(|| not_found("rate plan not found"))?;

        rate_plan.is_active = is_active;

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
