use chrono::NaiveDate;
use pms_rs::domain::inventory::HotelInventory;

#[test]
fn oversell_visible_per_day() {
    let mut inv = HotelInventory::new(10);
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();
    inv.add_reservation(date,12);
    assert_eq!(inv.oversolved(date), 2);
}

#[test]
fn undersell_visible_per_day() {
    let mut inv = HotelInventory::new(10);
    let date = NaiveDate::from_ymd_opt(2026, 5, 1).unwrap();

    inv.add_reservation(date, 8);
    assert_eq!(inv.oversolved(date), -2)
}