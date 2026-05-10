use chrono::{DateTime, Utc};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceStatus {
    Issued,
    Voided,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub billing_account_id: Uuid,
    pub issued_amount: i64,
    pub issued_at: DateTime<Utc>,
    pub status: InvoiceStatus,
}

impl Invoice {

    pub fn issue(
        id: Uuid,
        folio_id: Uuid,
        billing_account_id: Uuid,
        issued_amount: i64,
    ) -> Result<Self, String> {

        if issued_amount < 0 {
            return Err(
                "issued amount cannot be negative".into()
            );
        }

        Ok(Self {
            id,
            folio_id,
            billing_account_id,
            issued_amount,
            issued_at: Utc::now(),
            status: InvoiceStatus::Issued,
        })
    }

    pub fn void(&mut self)
        -> Result<(), String> {

        if self.status == InvoiceStatus::Voided {
            return Err(
                "invoice already voided".into()
            );
        }

        self.status =
            InvoiceStatus::Voided;

        Ok(())
    }
}