use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use super::operation_context::{OperationActor, OperationSource};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedField {
    pub field_name: String,
    pub before_value: Option<String>,
    pub after_value: Option<String>,
}

impl ChangedField {
    pub fn new(
        field_name: impl Into<String>,
        before_value: Option<String>,
        after_value: Option<String>,
    ) -> Self {
        Self {
            field_name: field_name.into(),
            before_value,
            after_value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Create,
    Modify,
    Cancel,
    NoShow,
    Reinstate,

    PostCharge,
    ReceivePayment,
    ApplyPayment,
    ReceiveDeposit,
    AdjustCharge,
    
    AssignBillingAccount,
    
    IssueInvoice,
    VoidInvoice,
    
    RefundPayment,

    CloseFolio,
    ReopenFolio,
    
    WriteOffReceivable,
    DisputeReceivable,
    ResolveReceivableDispute,

    AllocateReceivablePayment,
    ReversePaymentAllocation,
}

impl OperationType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Modify => "modify",
            Self::Cancel => "cancel",
            Self::NoShow => "no_show",
            Self::Reinstate => "reinstate",

            Self::PostCharge => "post_charge",
            Self::ReceivePayment => "receive_payment",
            Self::ApplyPayment => "apply_payment",
            Self::ReceiveDeposit => "receive_deposit",
            Self::AdjustCharge => "adjust_charge",

            Self::AssignBillingAccount => "assign_billing_account",

            Self::IssueInvoice => "issue_invoice",
            Self::VoidInvoice => "void_invoice",

            Self::RefundPayment => "refund_payment",

            Self::CloseFolio => "close_folio",
            Self::ReopenFolio => "reopen_folio",

            Self::WriteOffReceivable => "write_off_receivable",
            Self::DisputeReceivable => "dispute_receivable",
            Self::ResolveReceivableDispute => "resolve_receivable_dispute",

            Self::AllocateReceivablePayment => "allocate_receivable_payment",
            Self::ReversePaymentAllocation => "reverse_payment_allocation",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "create" => Some(Self::Create),
            "modify" => Some(Self::Modify),
            "cancel" => Some(Self::Cancel),
            "no_show" => Some(Self::NoShow),
            "reinstate" => Some(Self::Reinstate),

            "post_charge" => Some(Self::PostCharge),
            "receive_payment" => Some(Self::ReceivePayment),
            "apply_payment" => Some(Self::ApplyPayment),
            "receive_deposit" => Some(Self::ReceiveDeposit),
            "adjust_charge" => Some(Self::AdjustCharge),

            "issue_invoice" => Some(Self::IssueInvoice),
            "void_invoice" => Some(Self::VoidInvoice),
            "refund_payment" => Some(Self::RefundPayment),

            "assign_billing_account" => Some(Self::AssignBillingAccount),
            
            "close_folio" => Some(Self::CloseFolio),
            "reopen_folio" => Some(Self::ReopenFolio),
            
            "write_off_receivable" => Some(Self::WriteOffReceivable),
            "dispute_receivable" => Some(Self::DisputeReceivable),
            "resolve_receivable_dispute" => Some(Self::ResolveReceivableDispute),

            "allocate_receivable_payment" => Some(Self::AllocateReceivablePayment),
            "reverse_payment_allocation" => Some(Self::ReversePaymentAllocation),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OperationChangeEvent {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub operation_type: OperationType,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,
    pub occurred_at: DateTime<Utc>,
}
