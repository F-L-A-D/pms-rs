use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Reservation {
    pub id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
}