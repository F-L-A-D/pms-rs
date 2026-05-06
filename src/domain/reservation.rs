use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub enum ReservationStatus {
    Active,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StayStatus{
    Confirmed,
    CheckedIn,
    CheckedOut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reservation {
    pub id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
    pub room_class: String,
    pub room_id: Option<String>,
}

impl Reservation {
    pub fn new(
        id: String,
        check_in: NaiveDate,
        check_out: NaiveDate,
        room_class: String,
    ) -> Result<Self, String> {
        if check_in > check_out {
            return Err("check_in must be before or equal to check_out".into());
        }
        Ok(Self {
            id,
            check_in,
            check_out,
            reservation_status: ReservationStatus::Active,
            stay_status: Some(StayStatus::Confirmed),
            room_class,
            room_id: None,
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