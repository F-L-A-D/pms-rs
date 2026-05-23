use chrono::Utc;

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::{ChangedField, OperationType},
        semantic_activation::SemanticActivationKey,
    },
    error::app_error::{not_found, AppResult},
    projection::signal::model::semantic_activation::SemanticActivation,
    repository::sqlite::operational::operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
};

pub async fn materialize_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<SemanticActivation> {
    let event = SqliteOperationChangeEventRepository::find_by_id(tx, event_id)
        .await?
        .ok_or_else(|| not_found("operation change event not found"))?;

    let changed_fields = parse_changed_fields(&event.changed_fields_json);

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
        OperationType::NoShow => (
            SemanticActivationKey::ReservationMarkedNoShow,
            Decimal::new(80, 2),
            Decimal::new(95, 2),
        ),
        OperationType::Reinstate => (
            SemanticActivationKey::ReservationReinstated,
            Decimal::new(90, 2),
            Decimal::new(90, 2),
        ),
        OperationType::Modify => {
            if has_changed_field(&changed_fields, &["check_in", "check_out", "room_class"]) {
                (
                    SemanticActivationKey::InventoryRelevantChange,
                    Decimal::new(90, 2),
                    Decimal::new(85, 2),
                )
            } else if has_changed_field(&changed_fields, &["daily_details"]) {
                (
                    SemanticActivationKey::KpiRelevantChange,
                    Decimal::new(85, 2),
                    Decimal::new(85, 2),
                )
            } else if has_changed_field(
                &changed_fields,
                &["package_breakdowns", "daily_revenue_allocations"],
            ) {
                (
                    SemanticActivationKey::BillingRelevantChange,
                    Decimal::new(80, 2),
                    Decimal::new(85, 2),
                )
            } else if has_changed_field(&changed_fields, &["participants"]) {
                (
                    SemanticActivationKey::GuestRelevantChange,
                    Decimal::new(75, 2),
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

        OperationType::PostCharge
        | OperationType::ApplyPayment
        | OperationType::ReceiveDeposit
        | OperationType::AdjustCharge
        | OperationType::AssignBillingAccount
        | OperationType::ApplyDepositToReceivable
        | OperationType::IssueInvoice
        | OperationType::VoidInvoice
        | OperationType::RefundPayment
        | OperationType::CloseFolio
        | OperationType::ReopenFolio
        | OperationType::WriteOffReceivable
        | OperationType::DisputeReceivable
        | OperationType::ResolveReceivableDispute
        | OperationType::AllocateReceivablePayment
        | OperationType::AllocateExistingPayment
        | OperationType::ReceivePayment
        | OperationType::ReversePaymentAllocation => (
            SemanticActivationKey::BillingRelevantChange,
            Decimal::new(80, 2),
            Decimal::new(90, 2),
        ),
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
