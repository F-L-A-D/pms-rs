use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ReceivableStatus {
    Open,
    Settled,
    Disputed,
    WrittenOff,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receivable {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub outstanding_amount: i64,
    pub status: ReceivableStatus,
}

impl Receivable {

    pub fn new(
        id: Uuid,
        invoice_id: Uuid,
        outstanding_amount: i64,
    ) -> Result<Self, String> {

        if outstanding_amount < 0 {
            return Err(
                "outstanding amount cannot be negative".into()
            );
        }

        Ok(Self {
            id,
            invoice_id,
            outstanding_amount,
            status: ReceivableStatus::Open,
        })
    }

    pub fn settle(
        &mut self
    ) -> Result<(), String> {

        if self.outstanding_amount != 0 {
            return Err(
                "cannot settle receivable with outstanding balance".into()
            );
        }

        self.status =
            ReceivableStatus::Settled;

        Ok(())
    }

    pub fn mark_disputed(
        &mut self
    ) {
        self.status =
            ReceivableStatus::Disputed;
    }

    pub fn write_off(
        &mut self
    ) {
        self.status =
            ReceivableStatus::WrittenOff;
    }
}