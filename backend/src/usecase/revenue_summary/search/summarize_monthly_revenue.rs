use crate::{
    db::connection::Db,
    domain::semantic::revenue_summary::RevenueSummaryLine,
    error::app_error::{infra, validation, AppResult},
    repository::sqlite::operational::reservation::revenue_summary_repository::SqliteRevenueSummaryRepository,
};

pub async fn execute(db: &Db, year_month: String) -> AppResult<Vec<RevenueSummaryLine>> {
    if year_month.len() != 7 {
        return Err(validation("year_month must be YYYY-MM"));
    }

    let mut tx = db.begin_tx().await;

    let result = SqliteRevenueSummaryRepository::summarize_monthly(&mut tx, &year_month).await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
