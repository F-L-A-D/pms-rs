use chrono::Utc;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::OperationType, semantic_activation::ChangePatternType,
    },
    error::app_error::{not_found, AppResult},
    projection::signal::model::change_pattern::ChangePattern,
    repository::sqlite::operational::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<ChangePattern> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let changed_fields =
        serde_json::from_str::<Vec<String>>(&event.changed_fields_json).unwrap_or_default();

    let pattern_type = match event.operation_type {
        OperationType::Create => ChangePatternType::ReservationCreated,
        OperationType::Cancel => ChangePatternType::ReservationCancelled,
        OperationType::Modify => {
            if changed_fields.iter().any(|field| {
                matches!(
                    field.as_str(),
                    "check_in" | "check_out" | "room_class" | "daily_details"
                )
            }) {
                ChangePatternType::ReservationStayShapeChanged
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
