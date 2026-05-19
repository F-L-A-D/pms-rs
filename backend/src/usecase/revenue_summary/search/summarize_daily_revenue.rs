use chrono::NaiveDate;

use crate::{
    db::connection::Db,
    domain::semantic::revenue_summary::RevenueSummaryLine,
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::revenue_summary_repository::SqliteRevenueSummaryRepository,
};

pub async fn execute(db: &Db, service_date: NaiveDate) -> AppResult<Vec<RevenueSummaryLine>> {
    let mut tx = db.begin_tx().await;

    let result = SqliteRevenueSummaryRepository::summarize_daily(&mut tx, service_date).await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
