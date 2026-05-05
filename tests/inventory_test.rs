use pms_rs::domain::inventory::HotelInventory;

#[test]
fn oversell_visible() {
    let mut inv = HotelInventory::new(10);
    inv.add_reservation(12);
    assert_eq!(inv.oversolved(), 2);
}

#[test]
fn undersell_visible() {
    let mut inv = HotelInventory::new(10);
    inv.add_reservation(8);
    assert_eq!(inv.oversolved(), -2)
}