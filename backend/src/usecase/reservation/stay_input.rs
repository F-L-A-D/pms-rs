use chrono::{Days, NaiveDate};

#[derive(Debug, Clone)]
pub enum StayInput {
    CheckInAndNights {
        check_in: NaiveDate,
        nights: i64,
        room_class: String,
    },
    CheckInAndCheckOut {
        check_in: NaiveDate,
        check_out: NaiveDate,
        room_class: String,
    },
    CheckOutAndNights {
        check_out: NaiveDate,
        nights: i64,
        room_class: String,
    },
}

impl StayInput {
    pub fn room_class(&self) -> String {
        match self {
            StayInput::CheckInAndNights { room_class, .. }
            | StayInput::CheckInAndCheckOut { room_class, .. }
            | StayInput::CheckOutAndNights { room_class, .. } => room_class.clone(),
        }
    }
}

pub fn normalize(input: StayInput) -> Result<(NaiveDate, NaiveDate), String> {
    match input {
        StayInput::CheckInAndNights {
            check_in, nights, ..
        } => {
            if nights < 0 {
                return Err("nights must be >= 0".into());
            }

            let check_out = check_in
                .checked_add_days(Days::new(nights as u64))
                .ok_or("date overflow")?;

            Ok((check_in, check_out))
        }

        StayInput::CheckInAndCheckOut {
            check_in,
            check_out,
            ..
        } => {
            if check_in > check_out {
                return Err("check_in must be <= check_out".into());
            }

            Ok((check_in, check_out))
        }

        StayInput::CheckOutAndNights {
            check_out, nights, ..
        } => {
            if nights < 0 {
                return Err("nights must be >= 0".into());
            }

            let check_in = check_out
                .checked_sub_days(Days::new(nights as u64))
                .ok_or("date overflow")?;

            Ok((check_in, check_out))
        }
    }
}
