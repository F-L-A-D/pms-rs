use chrono::NaiveDate;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::domain::reservation::Reservation;
use pms_rs::usecase::create_reservation::create_reservation;

#[test]
fn reservation_spans_multiple_days() {
    let mut inv = HotelInventory::new(10);

    let res = Reservation{
        id: "r1".into(),
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        check_out: NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    };

    create_reservation(&mut inv, &res);
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&1));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 2).unwrap()), Some(&1));
}