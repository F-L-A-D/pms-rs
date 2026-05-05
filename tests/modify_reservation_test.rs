use chrono::NaiveDate;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::domain::reservation::Reservation;
use pms_rs::usecase::create_reservation::create_reservation;
use pms_rs::usecase::cancel_reservation::cancel_reservation;
use pms_rs::usecase::modify_reservation::modify_reservation;

#[test]
fn modify_should_update_inventory_dates() {
    let mut inv = HotelInventory::new(10);

    let mut res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);

    modify_reservation(
        &mut inv,
        &mut res,
        NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 4).unwrap(),
    );

    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&0));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 2).unwrap()), Some(&1));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 3).unwrap()), Some(&1));

    assert_eq!(res.check_in, NaiveDate::from_ymd_opt(2026, 5, 2).unwrap());
    assert_eq!(res.check_out, NaiveDate::from_ymd_opt(2026, 5, 4).unwrap());
}

#[test]
fn modify_should_extend_reservation() {
    let mut inv = HotelInventory::new(10);

    let mut res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);

    modify_reservation(
        &mut inv,
        &mut res,
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    );

    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&1));
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 2).unwrap()), Some(&1));
}

#[test]
fn modify_should_not_apply_if_cancelled() {
    let mut inv = HotelInventory::new(10);

    let mut res = Reservation::new(
        "r1".into(),
        NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
    ).unwrap();

    create_reservation(&mut inv, &res);

    // cancel
    cancel_reservation(&mut inv, &mut res);

    modify_reservation(
        &mut inv,
        &mut res,
        NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 3).unwrap(),
    );

    assert_eq!(res.check_in, NaiveDate::from_ymd_opt(2026, 5, 1).unwrap());
    assert_eq!(res.check_out, NaiveDate::from_ymd_opt(2026, 5, 2).unwrap());

    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&0));
}