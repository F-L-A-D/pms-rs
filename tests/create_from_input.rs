use chrono::NaiveDate;
use pms_rs::adapter::stay_input::StayInput;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::usecase::create_from_input::create_from_input;

#[test]
fn create_from_nights() {
    let mut inv = HotelInventory::new(10);

    let input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        nights: 2,
    };

    let res = create_from_input(&mut inv, "r1".into(), input).unwrap();

    assert_eq!(res.check_out, NaiveDate::from_ymd_opt(2026, 5, 3).unwrap());
}