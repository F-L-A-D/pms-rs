use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum FolioStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Folio {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub billing_account_id: Option<Uuid>,
    pub status: FolioStatus,
}

impl Folio {
    pub fn new(
        id: Uuid, 
        reservation_id: Uuid,
        billing_account_id: Option<Uuid>,
    ) -> Self {

        Self {
            id,
            reservation_id,
            billing_account_id,
            status: FolioStatus::Open,
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        if self.status == FolioStatus::Closed {
            return Err("folio already closed".into());
        }
        self.status = FolioStatus::Closed;

        Ok(())
    }

    pub fn assign_billing_account(
        &mut self,
        billing_account_id: Uuid,
    ) -> Result<(), String> {

        if self.status == FolioStatus::Closed {
            return Err(
                "cannot change billing responsibility on closed folio".into()
            );
        }

        self.billing_account_id =
            Some(billing_account_id);

        Ok(())
    }
}
