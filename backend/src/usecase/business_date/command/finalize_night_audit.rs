use chrono::{Duration, Utc};

use crate::{
    api::dto::business_date::input::finalize_night_audit_input::FinalizeNightAuditInput,
    db::connection::Db,
    domain::entity::business_date::BusinessDate,
    error::app_error::{conflict, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
    usecase::business_date::night_audit_worklist::collect_worklist,
};

pub struct FinalizeNightAuditResult {
    pub closed_business_date: BusinessDate,
    pub current_business_date: BusinessDate,
}

pub async fn execute(
    db: &Db,
    _input: FinalizeNightAuditInput,
) -> AppResult<FinalizeNightAuditResult> {
    let mut tx = db.begin_tx().await;

    let mut closing_business_date = SqliteBusinessDateRepository::find_current_closing(&mut tx)
        .await?
        .ok_or_else(|| conflict("closing business date not found"))?;

    let worklist = collect_worklist(&mut tx, closing_business_date.clone()).await?;

    if !worklist.unresolved_arrivals.is_empty() {
        let _ = tx.rollback().await;
        return Err(conflict("night audit has unresolved arrivals"));
    }

    if !worklist.unresolved_departures.is_empty() {
        let _ = tx.rollback().await;
        return Err(conflict("night audit has unresolved departures"));
    }

    if !worklist.room_charge_candidates.is_empty() {
        let _ = tx.rollback().await;
        return Err(conflict("night audit has unposted room charges"));
    }

    let now = Utc::now();

    closing_business_date.finalize_close(now)?;

    let next_date = closing_business_date.business_date + Duration::days(1);

    let next_business_date = BusinessDate::new_open(next_date, now);

    SqliteBusinessDateRepository::update(&mut tx, &closing_business_date).await?;

    SqliteBusinessDateRepository::insert(&mut tx, &next_business_date).await?;

    tx.commit().await.map_err(crate::error::app_error::infra)?;

    Ok(FinalizeNightAuditResult {
        closed_business_date: closing_business_date,
        current_business_date: next_business_date,
    })
}
