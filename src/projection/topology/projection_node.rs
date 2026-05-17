#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionNode {
    DailyRoomClassKpiAggregate,
    GuestAggregate,
    GuestActivitySignal,
    HousekeepingDailyWorkloadAggregate,
    InventoryAggregate,
}
