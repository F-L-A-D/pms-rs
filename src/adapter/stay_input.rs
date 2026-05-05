use chrono::{NaiveDate, Days};

#[derive(Debug, Clone)]
pub enum StayInput {
    CheckInAndNights {
        check_in: NaiveDate,
        nights: i64,
    },
    CheckInAndCheckOut {
        check_in: NaiveDate,
        check_out: NaiveDate,
    },
    CheckOutAndNights {
        check_out: NaiveDate,
        nights: i64,
    },
}

pub fn normalize(input: StayInput) -> Result<(NaiveDate, NaiveDate), String> {
    match input {
        StayInput::CheckInAndNights { check_in, nights } => {
            if nights < 0 {
                return Err("nights must be >= 0".into());
            }

            let check_out = check_in
                .checked_add_days(Days::new(nights as u64))
                .ok_or("date overflow")?;

            Ok((check_in, check_out))
        }

        StayInput::CheckInAndCheckOut { check_in, check_out } => {
            if check_in > check_out {
                return Err("check_in must be <= check_out".into());
            }

            Ok((check_in, check_out))
        }

        StayInput::CheckOutAndNights { check_out, nights } => {
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