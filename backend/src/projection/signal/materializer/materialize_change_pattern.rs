use chrono::Utc;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::{ChangedField, OperationType},
        semantic_activation::ChangePatternType,
    },
    error::app_error::{not_found, AppResult},
    projection::signal::model::change_pattern::ChangePattern,
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<ChangePattern> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let changed_fields = parse_changed_fields(&event.changed_fields_json);

    let pattern_type = match event.operation_type {
        OperationType::Create => ChangePatternType::ReservationCreated,
        OperationType::Cancel => ChangePatternType::ReservationCancelled,
        OperationType::NoShow => ChangePatternType::ReservationMarkedNoShow,
        OperationType::Reinstate => ChangePatternType::ReservationReinstated,
        OperationType::Modify => {
            if has_changed_field(&changed_fields, &["check_in", "check_out"]) {
                ChangePatternType::ReservationDateChanged
            } else if has_changed_field(&changed_fields, &["room_class"]) {
                ChangePatternType::ReservationRoomClassChanged
            } else if has_changed_field(&changed_fields, &["daily_details"]) {
                ChangePatternType::ReservationStayShapeChanged
            } else if has_changed_field(&changed_fields, &["plan_code"]) {
                ChangePatternType::ReservationDailyPlanChanged
            } else if has_changed_field(&changed_fields, &["participants"]) {
                ChangePatternType::ReservationGuestCompositionChanged
            } else if has_changed_field(
                &changed_fields,
                &["package_breakdowns", "daily_revenue_allocations"],
            ) {
                ChangePatternType::ReservationRevenueAllocationChanged
            } else {
                ChangePatternType::ReservationUpdated
            }
        }
    };

    Ok(ChangePattern {
        event_id,
        pattern_type,
        operation_type: event.operation_type,
        changed_fields_json: event.changed_fields_json,
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

fn has_changed_field(changed_fields: &[ChangedField], field_names: &[&str]) -> bool {
    changed_fields.iter().any(|field| {
        field_names.iter().any(|field_name| {
            field.field_name == *field_name
                || field
                    .field_name
                    .strip_prefix(field_name)
                    .is_some_and(|suffix| suffix.starts_with('.'))
        })
    })
}
