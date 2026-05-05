use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum ReservationStatus {
    Active,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reservation {
    pub id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub status: ReservationStatus,
}

impl Reservation {
    pub fn new(
        id: String,
        check_in: NaiveDate,
        check_out: NaiveDate,
    ) -> Result<Self, String> {
        if check_in > check_out {
            return Err("check_in must be before or equal to check_out".into());
        }
        Ok(Self {
            id,
            check_in,
            check_out,
            status: ReservationStatus::Active,
        })
    }

    pub fn nights(&self) -> Vec<NaiveDate> {
        let mut dates = vec![];
        let mut current = self.check_in;

        while current < self.check_out {
            dates.push(current);
            current = current.succ_opt().unwrap();
        }
        dates
    }
}