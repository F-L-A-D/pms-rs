use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Reservation {
    pub id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
}

impl Reservation {
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