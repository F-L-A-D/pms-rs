use chrono::Utc;

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::operation_change_event::OperationType,
    error::app_error::{not_found, AppResult},
    projection::signal::model::confidence_profile::ConfidenceProfile,
    repository::sqlite::operational::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<ConfidenceProfile> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let confidence_score = match event.operation_type {
        OperationType::Create => Decimal::new(100, 2),
        OperationType::Modify => Decimal::new(85, 2),
        OperationType::Cancel => Decimal::new(95, 2),
    };

    let reasons_json = serde_json::json!([
        {
            "rule": "fixed_operation_type_baseline",
            "operation_type": event.operation_type.to_snake()
        }
    ])
    .to_string();

    Ok(ConfidenceProfile {
        event_id,
        confidence_score,
        reasons_json,
        projection_version: 1,
        updated_at: Utc::now(),
    })
}
