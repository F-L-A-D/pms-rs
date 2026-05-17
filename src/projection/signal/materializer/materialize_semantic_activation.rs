use chrono::Utc;

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::OperationType, semantic_activation::SemanticActivationKey,
    },
    error::app_error::{not_found, AppResult},
    projection::signal::model::semantic_activation::SemanticActivation,
    repository::sqlite::operational::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<SemanticActivation> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let changed_fields =
        serde_json::from_str::<Vec<String>>(&event.changed_fields_json).unwrap_or_default();

    let (activation_key, activation_score, confidence_score) = match event.operation_type {
        OperationType::Create => (
            SemanticActivationKey::ReservationCreated,
            Decimal::new(80, 2),
            Decimal::new(100, 2),
        ),
        OperationType::Cancel => (
            SemanticActivationKey::ReservationCancelled,
            Decimal::new(90, 2),
            Decimal::new(95, 2),
        ),
        OperationType::Modify => {
            if changed_fields.iter().any(|field| {
                matches!(
                    field.as_str(),
                    "check_in" | "check_out" | "room_class" | "daily_details"
                )
            }) {
                (
                    SemanticActivationKey::StayShapeChanged,
                    Decimal::new(90, 2),
                    Decimal::new(85, 2),
                )
            } else {
                (
                    SemanticActivationKey::ReservationUpdated,
                    Decimal::new(60, 2),
                    Decimal::new(85, 2),
                )
            }
        }
    };

    Ok(SemanticActivation {
        event_id,
        activation_key,
        activation_score,
        confidence_score,
        is_active: true,
        projection_version: 1,
        updated_at: Utc::now(),
    })
}
