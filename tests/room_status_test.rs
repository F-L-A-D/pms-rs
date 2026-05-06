use pms_rs::domain::room::{Room, OccupancyStatus, HousekeepingStatus};

#[test]
fn check_in_should_succeed_when_room_ready() {
    let mut room = Room {
        id: "101".into(),
        room_type: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Inspected,
    };

    assert!(room.check_in().is_ok());
    assert_eq!(room.occupancy_status, OccupancyStatus::Occupied);
}

#[test]
fn check_in_should_fail_when_not_inspected() {
    let mut room = Room {
        id: "101".into(),
        room_type: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Dirty,
    };

    assert!(room.check_in().is_err());
}

#[test]
fn cleaning_flow_should_work() {
    let mut room = Room {
        id: "101".into(),
        room_type: "single".into(),
        occupancy_status: OccupancyStatus::Vacant,
        housekeeping_status: HousekeepingStatus::Dirty,
    };

    room.start_cleaning().unwrap();
    room.finish_cleaning().unwrap();
    room.inspect().unwrap();

    assert_eq!(room.housekeeping_status, HousekeepingStatus::Inspected);
}