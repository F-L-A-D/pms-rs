use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

use serde::Serialize;

use uuid::Uuid;

use crate::domain::{
    entity::payment::PaymentMethod,
    semantic::{
        operation_change_event::OperationType,
        operation_context::{OperationActor, OperationSource},
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct BillingAuditResponse {
    pub operation_id: Uuid,

    pub folio_id: Option<Uuid>,
    pub folio_entry_id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,

    pub aggregate_type: String,
    pub aggregate_id: Uuid,

    pub operation_type: OperationType,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,

    pub action: Option<String>,
    pub reason: Option<String>,

    pub amount: Option<Decimal>,
    pub payment_method: Option<PaymentMethod>,
    pub payment_reference: Option<String>,

    pub invoice_number: Option<String>,
    pub issued_amount: Option<Decimal>,

    pub billing_account_id: Option<Uuid>,
    pub billing_account_name: Option<String>,

    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,

    pub occurred_at: DateTime<Utc>,
}
