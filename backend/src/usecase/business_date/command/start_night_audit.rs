use chrono::Utc;

use crate::{
    api::dto::business_date::input::start_night_audit_input::StartNightAuditInput,
    db::connection::Db,
    domain::entity::business_date::BusinessDate,
    error::app_error::{conflict, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
};

pub async fn execute(db: &Db, _input: StartNightAuditInput) -> AppResult<BusinessDate> {
    let mut tx = db.begin_tx().await;

    let mut business_date = SqliteBusinessDateRepository::find_current_open(&mut tx)
        .await?
        .ok_or_else(|| conflict("open business date not found"))?;

    let now = Utc::now();

    business_date.start_closing(now)?;

    SqliteBusinessDateRepository::update(&mut tx, &business_date).await?;

    tx.commit().await.map_err(crate::error::app_error::infra)?;

    Ok(business_date)
}
