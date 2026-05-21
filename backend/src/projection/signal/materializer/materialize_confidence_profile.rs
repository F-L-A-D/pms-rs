use chrono::Utc;

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::{ChangedField, OperationType},
        operation_context::OperationSource,
    },
    error::app_error::{not_found, AppResult},
    projection::signal::model::confidence_profile::ConfidenceProfile,
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<ConfidenceProfile> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let changed_fields = parse_changed_fields(&event.changed_fields_json);
    let structured_fields = serde_json::from_str::<Vec<ChangedField>>(&event.changed_fields_json)
        .map(|fields| !fields.is_empty())
        .unwrap_or(false);

    let base_score = match event.operation_type {
        OperationType::Create => Decimal::new(100, 2),
        OperationType::Modify => Decimal::new(85, 2),
        OperationType::Cancel => Decimal::new(95, 2),
        OperationType::NoShow => Decimal::new(95, 2),
        OperationType::Reinstate => Decimal::new(90, 2),
    };
    let source_penalty = match event.source {
        OperationSource::Api => Decimal::ZERO,
        OperationSource::Internal => Decimal::new(5, 2),
        OperationSource::Batch | OperationSource::Import => Decimal::new(10, 2),
    };
    let before_penalty = if matches!(
        event.operation_type,
        OperationType::Modify 
        | OperationType::Cancel 
        | OperationType::NoShow
        | OperationType::Reinstate
    ) && event.before_json.is_none()
    {
        Decimal::new(20, 2)
    } else {
        Decimal::ZERO
    };
    let changed_fields_penalty = if changed_fields.is_empty() {
        Decimal::new(15, 2)
    } else {
        Decimal::ZERO
    };
    let confidence_score = base_score - source_penalty - before_penalty - changed_fields_penalty;

    let reasons_json = serde_json::json!([
        {
            "rule": "fixed_operation_type_baseline",
            "operation_type": event.operation_type.to_snake(),
            "score": base_score.to_string()
        },
        {
            "rule": "source_reliability",
            "source": event.source.to_snake(),
            "penalty": source_penalty.to_string()
        },
        {
            "rule": "payload_completeness",
            "has_before": event.before_json.is_some(),
            "changed_field_count": changed_fields.len(),
            "penalty": (before_penalty + changed_fields_penalty).to_string()
        },
        {
            "rule": "semantic_specificity",
            "structured_changed_fields": structured_fields
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

fn parse_changed_fields(changed_fields_json: &str) -> Vec<ChangedField> {
    serde_json::from_str::<Vec<ChangedField>>(changed_fields_json).unwrap_or_else(|_| {
        serde_json::from_str::<Vec<String>>(changed_fields_json)
            .unwrap_or_default()
            .into_iter()
            .map(|field_name| ChangedField::new(field_name, None, None))
            .collect()
    })
}
