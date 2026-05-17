use rust_decimal::Decimal;

use uuid::Uuid;

use crate::domain::entity::payment::PaymentMethod;

#[derive(Debug, Clone)]
pub struct CreatePaymentInput {
    pub folio_id: Uuid,
    pub amount: Decimal,
    pub method: PaymentMethod,
    pub external_reference: Option<String>,
}
