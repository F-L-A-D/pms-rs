use chrono::NaiveDate;

use pms_rs::domain::inventory::HotelInventory;
use pms_rs::repository::in_memory_reservation_repository::InMemoryReservationRepository;
use pms_rs::transaction::transaction::run_in_transaction;

#[test]
fn transaction_should_rollback_on_error() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let result: Result<(), String> = run_in_transaction(&mut repo, &mut inv, |_repo, inv| {
        inv.add_reservation(
            NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
            1,
        );

        Err("fail".into())
    });

    assert!(result.is_err());

    assert_eq!(
        inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()),
        None
    );
}