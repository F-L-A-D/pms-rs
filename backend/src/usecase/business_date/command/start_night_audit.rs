use chrono::Utc;

use crate::{
    api::dto::business_date::input::start_night_audit_input::StartNightAuditInput,
    db::connection::Db,
    domain::entity::business_date::BusinessDate,
    domain::semantic::{operation_change_event::ChangedField, operation_context::OperationContext},
    error::app_error::{conflict, infra, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, _input: StartNightAuditInput) -> AppResult<BusinessDate> {
    let mut tx = db.begin_tx().await;

    let mut business_date = SqliteBusinessDateRepository::find_current_open(&mut tx)
        .await?
        .ok_or_else(|| conflict("open business date not found"))?;
    let before = business_date.clone();

    let now = Utc::now();

    business_date.start_closing(now)?;

    SqliteBusinessDateRepository::update(&mut tx, &business_date).await?;

    let context = OperationContext::api_system();
    record_audit_log(
        &mut tx,
        &context,
        RecordAuditLogInput {
            aggregate_type: "business_date".to_string(),
            aggregate_id: business_date.id,
            action: "night_audit.start".to_string(),
            before_json: Some(business_date_json(&before).to_string()),
            after_json: business_date_json(&business_date).to_string(),
            changed_fields_json: serde_json::to_string(&vec![ChangedField::new(
                "status",
                Some(before.status.to_snake().to_string()),
                Some(business_date.status.to_snake().to_string()),
            )])
            .map_err(infra)?,
            reason: None,
        },
    )
    .await?;

    tx.commit().await.map_err(crate::error::app_error::infra)?;

    Ok(business_date)
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
