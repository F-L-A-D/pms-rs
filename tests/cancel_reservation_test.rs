use chrono::NaiveDate;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::domain::reservation::{Reservation, ReservationStatus};
use pms_rs::usecase::create_reservation::create_reservation;
use pms_rs::usecase::cancel_reservation::cancel_reservation;

#[test]
fn cancel_should_restore_inventory() {
    let mut inv = HotelInventory::new(10);

    let mut res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);

    cancel_reservation(&mut inv, &mut res);

    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&0));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 2).unwrap()), Some(&0));
    assert_eq!(res.status, ReservationStatus::Cancelled);
}

#[test]
fn cancel_should_be_idempotent() {
    let mut inv = HotelInventory::new(10);

    let mut res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);

    cancel_reservation(&mut inv, &mut res);
    cancel_reservation(&mut inv, &mut res);

    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&0));
}