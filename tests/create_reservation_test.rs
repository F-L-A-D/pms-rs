use chrono::NaiveDate;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::domain::reservation::Reservation;
use pms_rs::usecase::create_reservation::create_reservation;

#[test]
fn reservation_spans_multiple_days() {
    let mut inv = HotelInventory::new(10);
    
    let res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);
    
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&1));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 2).unwrap()), Some(&1));
}

#[test]
fn invalid_date_range_should_fail() {
    let res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
    );

    assert!(res.is_err());
}

#[test]
fn day_use_should_have_zero_nights() {
    let res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
    ).unwrap();

    assert_eq!(res.nights().len(), 0);
}