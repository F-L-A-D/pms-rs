use chrono::NaiveDate;

use pms_rs::adapter::stay_input::StayInput;
use pms_rs::domain::inventory::HotelInventory;
use pms_rs::repository::reservation_repository::ReservationRepository;
use pms_rs::repository::in_memory_reservation_repository::InMemoryReservationRepository;
use pms_rs::usecase::create_with_repo::create_with_repo;
use pms_rs::usecase::modify_with_repo::modify_with_repo;
use pms_rs::usecase::cancel_with_repo::cancel_with_repo;

#[test]
fn create_with_repo_should_persist_reservation() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        nights: 2,
    };

    create_with_repo(&mut repo, &mut inv, "r1".into(), input).unwrap();

    let res = repo.find_by_id("r1").unwrap();

    assert_eq!(res.check_in, NaiveDate::from_ymd_opt(2026, 5, 1).unwrap());
    assert_eq!(res.check_out, NaiveDate::from_ymd_opt(2026, 5, 3).unwrap());
}

#[test]
fn modify_with_repo_should_update_reservation() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        nights: 2,
    };

    create_with_repo(&mut repo, &mut inv, "r1".into(), input).unwrap();

    let modify_input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 2).unwrap(),
        nights: 2,
    };

    modify_with_repo(&mut repo, &mut inv, "r1", modify_input).unwrap();

    let res = repo.find_by_id("r1").unwrap();

    assert_eq!(res.check_in, NaiveDate::from_ymd_opt(2026, 5, 2).unwrap());
    assert_eq!(res.check_out, NaiveDate::from_ymd_opt(2026, 5, 4).unwrap());
}

#[test]
fn modify_should_fail_if_not_found() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        nights: 2,
    };

    let result = modify_with_repo(&mut repo, &mut inv, "unknown", input);

    assert!(result.is_err());
}

#[test]
fn cancel_with_repo_should_update_status_and_inventory() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let input = StayInput::CheckInAndNights {
        check_in: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        nights: 2,
    };

    create_with_repo(&mut repo, &mut inv, "r1".into(), input).unwrap();

    cancel_with_repo(&mut repo, &mut inv, "r1").unwrap();

    let res = repo.find_by_id("r1").unwrap();

    assert_eq!(res.status, pms_rs::domain::reservation::ReservationStatus::Cancelled);
    assert_eq!(inv.reserved.get(&NaiveDate::from_ymd_opt(2026, 5, 1).unwrap()), Some(&0));
}

#[test]
fn cancel_should_fail_if_not_found() {
    let mut repo = InMemoryReservationRepository::new();
    let mut inv = HotelInventory::new(10);

    let result = cancel_with_repo(&mut repo, &mut inv, "unknown");

    assert!(result.is_err());
}