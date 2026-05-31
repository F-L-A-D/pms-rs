use crate::{
    db::connection::Db,
    error::app_error::{conflict, infra, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
    usecase::business_date::night_audit_worklist::{collect_worklist, NightAuditWorklist},
};

pub async fn execute(db: &Db) -> AppResult<NightAuditWorklist> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let business_date = SqliteBusinessDateRepository::find_current_active(&mut tx)
            .await?
            .ok_or_else(|| conflict("active business date not found"))?;

        collect_worklist(&mut tx, business_date).await
    }
    .await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
