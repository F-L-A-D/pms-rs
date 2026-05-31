use chrono::{Duration, Utc};

use crate::{
    api::dto::business_date::input::finalize_night_audit_input::FinalizeNightAuditInput,
    db::connection::Db,
    domain::entity::business_date::BusinessDate,
    domain::semantic::{operation_change_event::ChangedField, operation_context::OperationContext},
    error::app_error::{conflict, infra, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
    usecase::{
        audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
        business_date::night_audit_worklist::collect_worklist,
    },
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
    let before = closing_business_date.clone();

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

    if !worklist.room_charge_blockers.is_empty() {
        let _ = tx.rollback().await;
        return Err(conflict("night audit has room charge blockers"));
    }

    let now = Utc::now();

    closing_business_date.finalize_close(now)?;

    let next_date = closing_business_date.business_date + Duration::days(1);

    let next_business_date = BusinessDate::new_open(next_date, now);

    SqliteBusinessDateRepository::update(&mut tx, &closing_business_date).await?;

    SqliteBusinessDateRepository::insert(&mut tx, &next_business_date).await?;

    let context = OperationContext::api_system();
    record_audit_log(
        &mut tx,
        &context,
        RecordAuditLogInput {
            aggregate_type: "business_date".to_string(),
            aggregate_id: closing_business_date.id,
            action: "night_audit.finalize".to_string(),
            before_json: Some(business_date_json(&before).to_string()),
            after_json: serde_json::json!({
                "closed_business_date": business_date_json(&closing_business_date),
                "current_business_date": business_date_json(&next_business_date),
            })
            .to_string(),
            changed_fields_json: serde_json::to_string(&vec![ChangedField::new(
                "status",
                Some(before.status.to_snake().to_string()),
                Some(closing_business_date.status.to_snake().to_string()),
            )])
            .map_err(infra)?,
            reason: None,
        },
    )
    .await?;

    tx.commit().await.map_err(crate::error::app_error::infra)?;

    Ok(FinalizeNightAuditResult {
        closed_business_date: closing_business_date,
        current_business_date: next_business_date,
    })
}

fn business_date_json(business_date: &BusinessDate) -> serde_json::Value {
    serde_json::json!({
        "id": business_date.id,
        "business_date": business_date.business_date,
        "status": business_date.status,
        "opened_at": business_date.opened_at,
        "closing_started_at": business_date.closing_started_at,
        "closed_at": business_date.closed_at,
    })
}
