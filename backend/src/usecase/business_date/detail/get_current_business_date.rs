use crate::{
    db::connection::Db,
    domain::entity::business_date::BusinessDate,
    error::app_error::{not_found, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
};

pub async fn execute(db: &Db) -> AppResult<BusinessDate> {
    let mut tx = db.begin_tx().await;

    let business_date = SqliteBusinessDateRepository::find_current_active(&mut tx)
        .await?
        .ok_or_else(|| not_found("current business date not found"))?;

    let _ = tx.rollback().await;

    Ok(business_date)
}
